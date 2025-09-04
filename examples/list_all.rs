use freewili_finder_rs::{FreeWiliDevice, FreeWiliError};

fn main() -> Result<(), FreeWiliError> {
    let devices = FreeWiliDevice::find_all()?;

    println!("Found {} FreeWili devices", devices.len());

    for (i, device) in devices.iter().enumerate() {
        let dev_type = device.device_type()?;
        let type_name = device.device_type_name()?;
        let name = device.name()?;
        let serial = device.serial()?;
        let unique_id = device.unique_id()?;
        let standalone = device.standalone()?;
        let usb_devices = device.get_usb_devices()?;

        println!("{}. Found device: {}", i + 1, device);
        println!("\ttype: {:?}", dev_type);
        println!("\ttype name: {}", type_name);
        println!("\tname: {}", name);
        println!("\tserial: {}", serial);
        println!("\tunique ID: {}", unique_id);
        println!("\tstandalone: {}", standalone);
        println!("\tUSB devices ({}):", usb_devices.len());

        for (count, usb_device) in usb_devices.iter().enumerate() {
            println!("\t\t{}: {}", count + 1, usb_device.kind_name);
            println!("\t\t\tname: {}", usb_device.name);
            println!("\t\t\tserial: {}", usb_device.serial);
            println!("\t\t\tVID: {} PID: {}", usb_device.vid, usb_device.pid);
            println!("\t\t\tlocation: {}", usb_device.location);
            println!("\t\t\tport chain: {:?}", usb_device.port_chain);
            if let Some(path) = &usb_device.path {
                println!("\t\t\tpath: {}", path);
            }
            if let Some(port) = &usb_device.port {
                println!("\t\t\tport: {}", port);
            }
        }

        // Try to get specific USB devices - these may fail, so handle gracefully
        if let Ok(main_usb_device) = device.get_main_usb_device() {
            println!("\tMain USB device: {}", main_usb_device.kind_name);
        } else {
            println!("\tNo main USB device found");
        }

        if let Ok(display_usb_device) = device.get_display_usb_device() {
            println!("\tDisplay USB device: {}", display_usb_device.kind_name);
        } else {
            println!("\tNo display USB device found");
        }

        if let Ok(fpga_usb_device) = device.get_fpga_usb_device() {
            println!("\tFPGA USB device: {}", fpga_usb_device.kind_name);
        } else {
            println!("\tNo FPGA USB device found");
        }

        if let Ok(hub_usb_device) = device.get_hub_usb_device() {
            println!("\tHUB USB device: {}", hub_usb_device.kind_name);
        } else {
            println!("\tNo HUB USB device found");
        }
    }

    Ok(())
}
