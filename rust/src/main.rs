#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
mod writeHelper;
mod types;
mod test;
use crate::test::{test};
use crate::types::{AREA_1_PACKAGE, COMMS_PACKAGE, ODU_PACKAGE, SEVEN_SEGMENT_LETTER_LOOKUP, SIXTEEN_SEGMENT_LETTER_LOOKUP, UFC_PACKAGE};
use crate::writeHelper::{segments_to_hex, send_hex_string, write_package_to_ufc};
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

    let area_1 = AREA_1_PACKAGE{chars: vec!['X', 'Y','1','.','2','3','4','.','5']};
    let odu_1 = ODU_PACKAGE{ id: 1, is_selected: true, text: "HALO".to_string() };
    let odu_2 = ODU_PACKAGE{ id: 2, is_selected: false, text: "MARS".to_string() };
    let odu_3 = ODU_PACKAGE{ id: 3, is_selected: true, text: "DAS".to_string() };
    let odu_4 = ODU_PACKAGE{ id: 4, is_selected: false, text: "IST".to_string() };
    let odu_5 = ODU_PACKAGE{ id: 5, is_selected: true, text: "FALA".to_string() };
    let comms_pack_left = COMMS_PACKAGE{ is_left: true, char: 'A' };
    let comms_pack_right = COMMS_PACKAGE{ is_left: false, char: 'B' };
    let odus = vec![odu_1, odu_2, odu_3, odu_4, odu_5];
    let comms = vec![comms_pack_left,comms_pack_right];
    let test = UFC_PACKAGE{ area_1, odu: odus, comms: comms };

    write_package_to_ufc(test, &device);

    return Ok(());
}
