#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
mod writeHelper;
mod types;
mod test;
use crate::test::{test};
use crate::types::{AREA_1_PACKAGE, SIXTEEN_SEGMENT_LETTER_LOOKUP, UFC_PACKAGE};
use crate::writeHelper::{segments_to_hex, send_hex_string};
use anyhow::{ anyhow, Context, Result };
use hidapi::{HidApi, HidDevice};

const VID:u16 = 0x4098;
const PID:u16 = 0xBEDE;


fn find_device() -> Result<HidDevice> {
    let hid_api = HidApi::new().context("Failed to initialize HID API")?;

    // Prefer opening by product string; fall back to the first matching VID/PID.
    let dev_info = hid_api
        .device_list()
        .find(|d|
            d.vendor_id() == VID &&
            d.product_id() == PID &&
            d.product_string().unwrap_or("").contains("WINWING PFP-3N-CAPTAIN")
        )
        .or_else(|| {
            hid_api
                .device_list()
                .find(|d| d.vendor_id() == VID && d.product_id() == PID)
        })
        .ok_or_else(|| anyhow!("No HID interfaces with VID={:04X} PID={:04X} found by hidapi", VID, PID))?;

    let path = dev_info.path();

    let device = hid_api
        .open_path(path)
        .with_context(|| format!("Failed to open HID path {}", path.to_string_lossy()))?;

    Ok(device)
}




fn main() -> Result<()>{

    let device = find_device()?;

    // test();

    let mut test_vec: Vec<String> = Vec::new();
    for i in 3..=9{
        test_vec.push("A_".to_string() + &i.to_string());
        test_vec.push("B_".to_string() + &i.to_string());
        test_vec.push("C_".to_string() + &i.to_string());
        test_vec.push("D_".to_string() + &i.to_string());
        test_vec.push("E_".to_string() + &i.to_string());
        test_vec.push("F_".to_string() + &i.to_string());
        // test_vec.push("G_".to_string() + &i.to_string());
    }



    // let area_1: AREA_1_PACKAGE = {chars: vec!['X', 'Y'];};
    // let test:UFC_PACKAGE = {};

    // if let Some(segment_string) = SIXTEEN_SEGMENT_LETTER_LOOKUP.get("B") {
    //     for segment in segment_string.chars() {
    //         test_vec.push(segment.to_string() + "_Right");
    //     }
    // } else {
    //     panic!("No segment found for 'A'");
    // }


    let a = segments_to_hex(test_vec, "AREA_1".to_string());

    let _ = send_hex_string(&device, a, 0.1);

    return Ok(());
}
