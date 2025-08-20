mod writeHelper;
use crate::writeHelper::{segments_to_bits, send_hex_string, AREA_1_BITS, ODU_BITS};
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

fn bits_to_hex(bits: &String) -> String{
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
fn main() -> Result<()>{

    let device = find_device()?;

    // let prefix_left = "02d0be0000064c".to_string() + &"16".to_string();
    // let prefix_right = "02d0be0000064c".to_string() + &"17".to_string();
    // let bits = "1001 1111 1111 1111 1111 1111 1111 1111  1111 1111 1111 1111 1111 1111 1111 1111".to_string();
    // // let bits: &'static str = AREA_2_BITS
    // //     .get("O_1")
    // //     .copied()                    // turn Option<&&str> into Option<&str>
    // //     .context("unknown key 'O2'")?;
    // let bits = bits.replace(' ',"").to_string();
    // let bits_left = bits[0..=31].to_string();
    // let bits_right = bits[32..=63].to_string();

    // let suffix = "0000".to_string();
    // // 02d0be0000064c09 00 20 0c 00
    // let hex_bits_left = bits_to_hex(&bits_left);
    // let hex_bits_right = bits_to_hex(&bits_right);


    // let mut packet_vec:Vec<String> = Vec::new();
    // packet_vec.push(prefix_left + &hex_bits_left +& suffix);
    // packet_vec.push(prefix_right + &hex_bits_right +& suffix);

    // let _ = send_hex_string(&device, packet_vec, 0.1);

    let mut test_vec: Vec<String> = Vec::new();
    test_vec.push("E_1".to_string());
    test_vec.push("B_1".to_string());
    test_vec.push("L_1".to_string());
    test_vec.push("O_1".to_string());
    test_vec.push("H_1".to_string());
    test_vec.push("C_1".to_string());
    test_vec.push("I_1".to_string());
    test_vec.push("N_1".to_string());

    let test = segments_to_bits(test_vec, "AREA_1".to_string());
    println!("{:?}", test);
    return Ok(());
}
