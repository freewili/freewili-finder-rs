use freewili_finder_rs::{fw_stringtype_type, FreeWiliDevice, FreeWiliError};

fn main() -> Result<(), FreeWiliError> {
    // Find all FreeWili devices
    let devices = FreeWiliDevice::find_all()?;
    println!("Found {} FreeWili(s)", devices.len());

    for (device_index, device) in devices.iter().enumerate() {
        if !device.is_valid() {
            println!("{}. Invalid device", device_index + 1);
            continue;
        }
        println!(
            "{}. {} (Device Type: {})",
            device_index + 1,
            device,
            device.get_device_string(fw_stringtype_type)?
        );

        // Get USB devices
        match device.get_usb_devices() {
            Ok(usb_devices) => {
                for (usb_index, usb_device) in usb_devices.iter().enumerate() {
                    println!("        {}. {}", usb_index + 1, usb_device);
                }
            }
            Err(e) => {
                println!("        Error getting USB devices: {e}");
            }
        }
    }

    Ok(())
}
