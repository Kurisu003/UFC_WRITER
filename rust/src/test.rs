//✅❌

use crate::{types::{AREA_1_BITS, COMMS_BITS, ODU_BITS}, writeHelper::{segments_to_bits, segments_to_hex}};

fn test_lut(){
    let a1 = &AREA_1_BITS;
    let c = &COMMS_BITS;
    let odu = &ODU_BITS;

    let a1_case_1 = *a1.get(&"O_1").unwrap() == 97;
    let a1_case_2 = *a1.get(&"J_2").unwrap() == 80;

    let c_case_1 = *c.get(&"N_Left").unwrap() == 59;
    let c_case_2 = *c.get(&"E_Right").unwrap() == 30;

    let odu_case_1 = *odu.get(&"O_2").unwrap() == 77;
    let odu_case_2 = *odu.get(&"N_4").unwrap() == 33;

    if (a1_case_1 & a1_case_2){println!("Area_1 LUT: ✅");} else { println!("Area_1 LUT: ❌");};
    if (c_case_1 & c_case_2){println!("Comms LUT: ✅");} else { println!("Comms LUT: ❌");};
    if (odu_case_1 & odu_case_2){println!("ODU LUT: ✅");} else { println!("ODU LUT: ❌");};
}

fn segments_to_bits_test(){
    // Area 1
    let mut area_1_vec: Vec<String> = Vec::new();
    area_1_vec.push("O_1".to_string());
    area_1_vec.push("P_1".to_string());
    let area_1_bits = segments_to_bits(area_1_vec, "AREA_1".to_string());

    let area_1_case = area_1_bits == (("00".to_string(), "01".to_string(), "02".to_string(),), "10000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string());

    if (area_1_case){println!("Area_1 segments_to_bits: ✅");} else { println!("Area_1 segments_to_bits: ❌");};

    // ODU
    let mut odu_vec: Vec<String> = Vec::new();
    odu_vec.push("C_1".to_string());
    odu_vec.push("H_1".to_string());
    let odu_bits = segments_to_bits(odu_vec, "ODU_1".to_string());
    let odu_case = odu_bits == (("04".to_string(), "05".to_string(), "06".to_string(),), "00000011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string());

    if (odu_case){println!("ODU segments_to_bits: ✅");} else { println!("ODU segments_to_bits: ❌");};

    // Comms
    let mut comms_vec: Vec<String> = Vec::new();
    comms_vec.push("F_Left".to_string());
    comms_vec.push("N_Left".to_string());
    let comms_bits = segments_to_bits(comms_vec, "COMMS".to_string());

    let comms_case = comms_bits == (("16".to_string(), "17".to_string(), "FF".to_string(),), "00000000000000000000000000000000000000100000000100000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string());

    if (comms_case){println!("Comms segments_to_bits: ✅");} else { println!("Comms segments_to_bits: ❌");};
}

fn segments_to_hex_test(){
    // Area 1
    let mut area_1_vec: Vec<String> = Vec::new();
    area_1_vec.push("O_1".to_string());
    area_1_vec.push("P_1".to_string());

    let area_1_hex = segments_to_hex(area_1_vec, "AREA_1".to_string());
    let area_1_case = vec!["02d0be0000064c00080800000000", "02d0be0000064c01000000000000", "02d0be0000064c02000000000000"] == area_1_hex;
    if (area_1_case){println!("Area_1 segments_to_hex: ✅");} else { println!("Area_1 segments_to_hex: ❌");};

}

pub fn test(){
    test_lut();
    segments_to_bits_test();
    segments_to_hex_test();
}