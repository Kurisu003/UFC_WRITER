#![allow(non_snake_case)]
use anyhow::{ Context, Result };
use hidapi::{ HidDevice };
use std::thread;
use std::time::Duration;

use crate::types::{AREA_1_BITS, COMMS_BITS, ODU_BITS};

fn parse_hex_line(line: &str) -> Result<Vec<u8>> {
    // Strip comments after '#'
    let s = line.split('#').next().unwrap_or("").trim();
    if s.is_empty() {
        return Ok(Vec::new());
    }

    let mut bytes = Vec::new();

    // If the line has separators, parse token-by-token
    if s.chars().any(|c| c.is_whitespace() || c == ',') {
        for tok in s.split(|c: char| c.is_whitespace() || c == ',') {
            if tok.is_empty() { continue; }
            let t = tok.strip_prefix("0x").or_else(|| tok.strip_prefix("0X")).unwrap_or(tok);
            anyhow::ensure!(t.len() <= 2, "Token '{tok}' is not a single byte");
            let b = u8::from_str_radix(t, 16)
                .with_context(|| format!("Invalid hex token '{tok}'"))?;
            bytes.push(b);
        }
    } else {
        // No separators: parse as contiguous hex pairs
        let t = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
        anyhow::ensure!(t.len() % 2 == 0, "Hex string must have even length");
        for i in (0..t.len()).step_by(2) {
            let pair = &t[i..i + 2];
            let b = u8::from_str_radix(pair, 16)
                .with_context(|| format!("Invalid hex pair '{pair}'"))?;
            bytes.push(b);
        }
    }

    Ok(bytes)
}

fn le_bits_to_hex(bits: &String) -> String{
    // 1) Remove whitespace and validate
    let cleaned: String = bits.chars().filter(|c| !c.is_whitespace()).collect();
    assert!(cleaned.chars().all(|c| c == '0' || c == '1'), "bits must be 0/1 only");
    assert_eq!(cleaned.len() % 4, 0, "bit length must be a multiple of 4");

    // 2) Swap every adjacent 4-bit nibble (i.e., [n0,n1,n2,n3,...] -> [n1,n0,n3,n2,...])
    let mut reordered = String::with_capacity(cleaned.len());
    for i in (0..cleaned.len()).step_by(8) {
        let a = &cleaned[i..i + 4];
        let b = &cleaned[i + 4..i + 8];
        reordered.push_str(b);
        reordered.push_str(a);
    }

    // 3) Convert binary to hex, zero-padded to nibble width
    let value = u128::from_str_radix(&reordered, 2).expect("invalid binary");
    let width = reordered.len() / 4;
    let hex_part = format!("{:0width$x}", value, width = width);

    return hex_part;
}

// TODO: Make types for this
pub fn segments_to_bits(segments: Vec<String>, area: String) -> ((String, String, String), String) {
    let area_lut = match area.as_str() {
        s if s.starts_with("AREA_1") => &AREA_1_BITS,
        s if s.starts_with("ODU") => &ODU_BITS,
        s if s.starts_with("COMMS") => &COMMS_BITS,
        _ => panic!(),
    };

    let prefixes = match area.as_str(){
        s if s.starts_with("AREA_1") =>("00".to_string(), "01".to_string(), "02".to_string()),
        s if s.starts_with("ODU_1") => ("04".to_string(), "05".to_string(), "06".to_string()),
        s if s.starts_with("ODU_2") => ("08".to_string(), "09".to_string(), "0a".to_string()),
        s if s.starts_with("ODU_3") => ("0c".to_string(), "0d".to_string(), "0e".to_string()),
        s if s.starts_with("ODU_4") => ("10".to_string(), "11".to_string(), "12".to_string()),
        s if s.starts_with("ODU_5") => ("14".to_string(), "15".to_string(), "16".to_string()),
        s if s.starts_with("COMMS") => ("16".to_string(), "17".to_string(), "FF".to_string()), //FF not used
        _ => panic!(),
    };

    let mut bits: Vec<bool> = vec![false; 97];
    let mut bit_string = String::from("");

    // Set bits to true that need to be a 1
    for segment in segments {
        let shift_index = *area_lut.get(&segment).unwrap() as usize;
        bits[shift_index-1] = true;
    }

    // construct String from true and false values
    for i in (0..97).rev() {
        if(bits[i]){
            bit_string.push('1');
        }
        else{
            bit_string.push('0');
        }
    }

    // pad with 0s at end
    bit_string.push_str(&"0".repeat(128-97));
    println!("{:?}", bit_string);

    return (prefixes, bit_string);
}

pub fn segments_to_hex(segments: Vec<String>, area: String) -> Vec<String>{
    let ((pf_1, pf_2, pf_3), num) = segments_to_bits(segments, area);

    let bits_left = num[0..=31].to_string();
    let bits_middle = num[32..=63].to_string();
    let bits_right = num[64..=95].to_string();

    let suffix = "0000".to_string();

    let prefix_1 = "02d0be0000064c".to_string() + &pf_1;
    let prefix_2 = "02d0be0000064c".to_string() + &pf_2;
    let prefix_3 = "02d0be0000064c".to_string() + &pf_3;

    let hex_left_be   = format!("{}{}{}", prefix_1,   le_bits_to_hex(&bits_left),   suffix);
    let hex_mid_be    = format!("{}{}{}", prefix_2, le_bits_to_hex(&bits_middle), suffix);
    let hex_right_be  = format!("{}{}{}", prefix_3,  le_bits_to_hex(&bits_right),  suffix);

    let mut packet_vec:Vec<String> = Vec::new();
    packet_vec.push(hex_left_be);
    packet_vec.push(hex_mid_be);
    packet_vec.push(hex_right_be);

    return packet_vec;
}

pub fn send_hex_string(device: &HidDevice, hex_lines: Vec<String>, delay_secs: f32) -> Result<usize> {
    let mut sent = 0usize;

    for line in hex_lines {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let buf = parse_hex_line(line)?;
        if buf.is_empty() {
            continue;
        }

        // Write raw bytes (what `HidDevice::write` expects)
        let _n = device.write(&buf)?;
        sent += 1;

        thread::sleep(Duration::from_secs_f32(delay_secs));
    }

    Ok(sent)
}
