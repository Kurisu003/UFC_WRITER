#![allow(non_snake_case)]
use std::{collections::HashMap, thread::sleep, time::Duration};
use phf::phf_map;

use crate::{dcsBiosHelper::{send_button_press, send_button_state_press, send_message_to_dcsbios}, types::{AREA_1_PACKAGE, COMMS_PACKAGE, ODU_PACKAGE, UFC_PACKAGE}};


fn get_value_by_address(values: &HashMap<u16, [u8; 2]>, address: u16) -> [u8; 2] {
    values.get(&address).copied().unwrap_or([0, 0])
}

fn addrs_to_65k_num_as_string(values: &HashMap<u16, [u8; 2]>, adrs: Vec<u16>) -> String{
    let mut text = String::from("");


    for add in adrs{
        let raw = get_value_by_address(values, add);
        let formatted = (f32::from(u16::from_le_bytes(raw)) / 6553.5).round();
        text += &formatted.to_string();
    }

    return text;
}

fn map_byte_to_char(b: u8) -> char {
    match b {
        0xB0 => '.',                 // custom mapping
        0xA1 => 'x',                 // custom mapping
        0xAB => 'y',                 // custom mapping
        0xBB => 'z',                 // custom mapping
        0xA9 => 'u',                 // custom mapping
        0xAE => '^',                 // custom mapping
        0xB6 => '_',                 // custom mapping
        0xB1 => '~',                 // custom mapping
        0x20..=0x7E => b.to_ascii_uppercase() as char,    // printable ASCII
        _ => '?',                    // fallback for non-ASCII bytes
    }
}

pub fn get_string_by_addr_and_len(
    values: &HashMap<u16, [u8; 2]>,
    base_addr: u16,
    length: u16,                    // length in BYTES; each cell is 2 bytes
) -> String {
    let cells = (length / 2) as usize;
    let mut out = String::with_capacity(length as usize);

    for i in 0..cells {
        let addr = base_addr.wrapping_add((2 * i) as u16);
        let [b0, b1] = get_value_by_address(values, addr);
        out.push(map_byte_to_char(b0));
        out.push(map_byte_to_char(b1));
    }

    out
}

pub fn get_AV8B_UFC_PACKAGE(values: &HashMap<u16, [u8; 2]>) -> UFC_PACKAGE{

    let odu_1_string = get_string_by_addr_and_len(values, 0x795a, 4);
    let odu_2_string = get_string_by_addr_and_len(values, 0x7960, 4);
    let odu_3_string = get_string_by_addr_and_len(values, 0x7966, 4);
    let odu_4_string = get_string_by_addr_and_len(values, 0x796c, 4);
    let odu_5_string = get_string_by_addr_and_len(values, 0x7972, 4);

    let is_odu_1_selected = get_value_by_address(values, 0x7958)[0] == 58;
    let is_odu_2_selected = get_value_by_address(values, 0x795e)[0] == 58;
    let is_odu_3_selected = get_value_by_address(values, 0x7964)[0] == 58;
    let is_odu_4_selected = get_value_by_address(values, 0x796a)[0] == 58;
    let is_odu_5_selected = get_value_by_address(values, 0x7970)[0] == 58;

    let scratchpad_string = get_string_by_addr_and_len(values, 0x7976, 12);
    let mut area_1_string = scratchpad_string.as_bytes();
    let area_1_0 = area_1_string[0] as char;
    let area_1_1 = area_1_string[1] as char;

    let area_1_2 = area_1_string[5] as char;
    let area_1_3 = area_1_string[6] as char;
    let area_1_4 = area_1_string[7] as char;
    let area_1_5 = area_1_string[8] as char;
    let area_1_6 = area_1_string[9] as char;
    let area_1_7 = area_1_string[10] as char;
    let area_1_8 = area_1_string[11] as char;

    let comms_left =  get_string_by_addr_and_len(values, 0x7954, 2).as_bytes()[1] as char;
    let comms_right =  get_string_by_addr_and_len(values, 0x7956, 2).as_bytes()[1] as char;



    let area_1 = AREA_1_PACKAGE{chars: vec![area_1_0, area_1_1,area_1_2,area_1_3,area_1_4,area_1_5,area_1_6,area_1_7,area_1_8]};
    let odu_1 = ODU_PACKAGE{ id: 1, is_selected: is_odu_1_selected, text: odu_1_string };
    let odu_2 = ODU_PACKAGE{ id: 2, is_selected: is_odu_2_selected, text: odu_2_string };
    let odu_3 = ODU_PACKAGE{ id: 3, is_selected: is_odu_3_selected, text: odu_3_string };
    let odu_4 = ODU_PACKAGE{ id: 4, is_selected: is_odu_4_selected, text: odu_4_string };
    let odu_5 = ODU_PACKAGE{ id: 5, is_selected: is_odu_5_selected, text: odu_5_string };
    let comms_pack_left = COMMS_PACKAGE{ is_left: true, char: comms_left };
    let comms_pack_right = COMMS_PACKAGE{ is_left: false, char: comms_right };
    let odus = vec![odu_1, odu_2, odu_3, odu_4, odu_5];
    let comms = vec![comms_pack_left,comms_pack_right];
    return  UFC_PACKAGE{ area_1, odu: odus, comms: comms };

}

fn AH64D_isCpg(values:&HashMap<u16, [u8;2]>)->bool{
    return (u16::from_le_bytes(get_value_by_address(values, 0x8750))&0x0100) == 256;
}

pub fn get_AH64D_text(values: &HashMap<u16, [u8;2]>){
}

pub fn get_module_name(values: &HashMap<u16, [u8; 2]>) -> String{
    return get_string_by_addr_and_len(values, 0x0000, 24);
}