#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
mod writeHelper;
mod types;
mod test;
mod dcsBiosHelper;
mod moduleDataProcessor;

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use crate::dcsBiosHelper::{get_map, read_stream};
use crate::test::{test};
use crate::moduleDataProcessor::{get_AV8B_UFC_PACKAGE, get_module_name};
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
    thread::spawn(|| {
        if let Err(e) = read_stream() {
            eprintln!("read_stream error: {}", e);
        }
    });

    let area_1 = AREA_1_PACKAGE{chars: vec!['X', 'X','8','8','8','8','8','8','8']};
    let odu_1 = ODU_PACKAGE{ id: 1, is_selected: true, text: "UFC".to_string() };
    let odu_2 = ODU_PACKAGE{ id: 2, is_selected: false, text: "NOT".to_string() };
    let odu_3 = ODU_PACKAGE{ id: 3, is_selected: true, text: "YET".to_string() };
    let odu_4 = ODU_PACKAGE{ id: 4, is_selected: false, text: "INIT".to_string() };
    let odu_5 = ODU_PACKAGE{ id: 5, is_selected: true, text: "XXXX".to_string() };
    let comms_pack_left = COMMS_PACKAGE{ is_left: true, char: 'X' };
    let comms_pack_right = COMMS_PACKAGE{ is_left: false, char: 'X' };
    let odus = vec![odu_1, odu_2, odu_3, odu_4, odu_5];
    let comms = vec![comms_pack_left,comms_pack_right];
    let mut ufc_package = UFC_PACKAGE{ area_1, odu: odus, comms: comms };

    loop{
        let map_arc = get_map(); // clone Arc so it lives long enough

        let snapshot: HashMap<u16, [u8; 2]> = {
                let guard = match map_arc.lock() {
                    Ok(g) => g,
                    Err(p) => p.into_inner(),
                };
                guard.clone() // clones the whole HashMap
            };

        // yes, I know this gets called every 10ms
        // yes, I know its not performant
        let module_name = get_module_name(&snapshot);

        if(module_name.starts_with("AV8B")){
            ufc_package = get_AV8B_UFC_PACKAGE(&snapshot)
        }

        write_package_to_ufc(ufc_package.clone(), &device);
    }

    return Ok(());
}
