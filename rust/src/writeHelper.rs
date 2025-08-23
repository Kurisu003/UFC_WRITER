#![allow(non_snake_case)]
use anyhow::{ Context, Result };
use hidapi::{ HidDevice };
use phf::phf_map;
use std::thread;
use std::time::Duration;

pub static AREA_1_BITS: phf::Map<&'static str, u32> = phf_map! {
    // "Prefixes" => "00 01 02",
    "O_1"      => 97,
    "J_1"      => 96,
    "H_1"      => 95,
    "C_1"      => 94,
    "K_1"      => 93,
    "L_1"      => 92,
    "D_1"      => 91,
    "A_1"      => 90,
    "P_1"      => 89,
    "M_1"      => 88,
    "E_1"      => 87,
    "B_1"      => 86,
    "N_1"      => 85,
    "I_1"      => 84,
    "F_1"      => 83,
    "G_1"      => 82,

    "O_2"      => 81,
    "J_2"      => 80,
    "H_2"      => 79,
    "C_2"      => 78,
    "K_2"      => 77,
    "L_2"      => 76,
    "D_2"      => 75,
    "A_2"      => 74,
    "P_2"      => 73,
    "M_2"      => 72,
    "E_2"      => 71,
    "B_2"      => 70,
    "N_2"      => 69,
    "I_2"      => 68,
    "F_2"      => 67,
    "G_2"      => 66,

    "E_3"      => 65,
    "G_3"      => 64,
    "F_3"      => 63,
    "D_3"      => 62,
    "C_3"      => 61,
    "B_3"      => 60,
    "A_3"      => 59,

    "E_4"      => 58,
    "G_4"      => 57,
    "F_4"      => 56,
    "D_4"      => 55,
    "C_4"      => 54,
    "B_4"      => 53,
    "A_4"      => 52,

    "E_5"      => 51,
    "G_5"      => 50,
    "F_5"      => 40,
    "D_5"      => 39,
    "C_5"      => 38,
    "B_5"      => 37,
    "A_5"      => 36,

    "E_6"      => 35,
    "G_6"      => 34,
    "F_6"      => 33,
    "D_6"      => 32,
    "C_6"      => 31,
    "B_6"      => 30,
    "A_6"      => 29,

    "E_7"      => 28,
    "G_7"      => 27,
    "F_7"      => 26,
    "D_7"      => 25,
    "C_7"      => 24,
    "B_7"      => 23,
    "A_7"      => 22,

    "E_8"      => 21,
    "G_8"      => 20,
    "F_8"      => 19,
    "D_8"      => 18,
    "C_8"      => 17,
    "B_8"      => 16,
    "A_8"      => 15,

    "E_9"      => 14,
    "G_9"      => 13,
    "F_9"      => 12,
    "D_9"      => 11,
    "C_9"      => 10,
    "B_9"      => 9,
    "A_9"      => 8,
};

pub static ODU_BITS: phf::Map<&'static str, u32> = phf_map! {
    // "Prefixes_1" => "04 05 06",
    // "Prefixes_2" => "08 09 0a",
    // "Prefixes_3" => "0c 0d 0e",
    // "Prefixes_4" => "10 11 12",
    // "Prefixes_5" => "14 15 16",
    ":"  => 92,
    "O_1"=> 91,
    "J_1"=> 90,
    "H_1"=> 89,
    "C_1"=> 88,
    "K_1"=> 87,
    "L_1"=> 86,
    "D_1"=> 85,
    "A_1"=> 84,
    "P_1"=> 83,
    "M_1"=> 82,
    "E_1"=> 81,
    "B_1"=> 80,
    "N_1"=> 79,
    "I_1"=> 78,
    "F_1"=> 77,
    "G_1"=> 76,

    "O_2"=>75,
    "J_2"=>74,
    "H_2"=>73,
    "C_2"=>72,
    "K_2"=>71,
    "L_2"=>70,
    "D_2"=>69,
    "A_2"=>68,
    "P_2"=>67,
    "M_2"=>66,
    "E_2"=>65,
    "B_2"=>64,
    "N_2"=>63,
    "I_2"=>62,
    "F_2"=>61,
    "G_2"=>60,

    "O_3"=>59,
    "J_3"=>58,
    "H_3"=>57,
    "C_3"=>56,
    "K_3"=>55,
    "L_3"=>54,
    "D_3"=>53,
    "A_3"=>52,
    "P_3"=>51,
    "M_3"=>50,
    "E_3"=>49,
    "B_3"=>48,
    "N_3"=>47,
    "I_3"=>46,
    "F_3"=>45,
    "G_3"=>44,

    "O_4"=>43,
    "J_4"=>42,
    "H_4"=>41,
    "C_4"=>40,
    "K_4"=>39,
    "L_4"=>38,
    "D_4"=>37,
    "A_4"=>36,
    "P_4"=>35,
    "M_4"=>34,
    "E_4"=>33,
    "B_4"=>32,
    "N_4"=>31,
    "I_4"=>30,
    "F_4"=>29,
    "G_4"=>28,
};

pub static COMMS_BITS: phf::Map<&'static str, u32> = phf_map! {
    // "Prefixes" => "16 17",
    "N_Left" =>59,
    "P_Left" =>58,
    "L_Left" =>57,
    "O_Left" =>56,
    "M_Left" =>55,
    "I_Left" =>54,
    "K_Left" =>53,
    "J_Left" =>52,
    "G_Left" =>51,
    "F_Left" =>50,
    "H_Left" =>49,
    "C_Left" =>48,
    "B_Left" =>47,
    "E_Left" =>46,
    "D_Left" =>45,
    "A_Left" =>44,
    "N_Right"=>43,
    "P_Right"=>42,
    "L_Right"=>41,
    "O_Right"=>40,
    "M_Right"=>39,
    "I_Right"=>38,
    "K_Right"=>37,
    "J_Right"=>36,
    "G_Right"=>35,
    "F_Right"=>34,
    "H_Right"=>33,
    "C_Right"=>32,
    "B_Right"=>31,
    "E_Right"=>30,
    "D_Right"=>29,
    "A_Right"=>28,
};


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

// TODO: Make types for this
pub fn segments_to_bits(segments: Vec<String>, area: String) -> u128 {
    let area_lut = match area.as_str() {
        s if s.starts_with("AREA_1") => &AREA_1_BITS,
        s if s.starts_with("ODU") => &ODU_BITS,
        s if s.starts_with("COMMS") => &COMMS_BITS,
        _ => panic!(),
    };

    let prefixes = match area.as_str(){
        s if s.starts_with("AREA_1") => (0x00, 0x01, 0x02),
        s if s.starts_with("ODU_1") => (0x04, 0x05, 0x06),
        s if s.starts_with("ODU_2") => (0x08, 0x09, 0x0a),
        s if s.starts_with("ODU_3") => (0x0c, 0x0d, 0x0e),
        s if s.starts_with("ODU_4") => (0x00, 0x01, 0x02),
        s if s.starts_with("ODU_5") => (0x00, 0x01, 0x02),
        s if s.starts_with("COMMS") => (0x00, 0x01, 0x02),
        _ => panic!(),
    };

    let mut bits: u128 = 0;
    for segment in segments {
        let shift_amount = area_lut.get(&segment).unwrap();
        let mut num: u128 = 1;
        num = num<<*shift_amount;
        bits = bits | num;
    }
    return bits;
}

pub fn update_area_1(text: &String){

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
