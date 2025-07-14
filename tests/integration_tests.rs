//! Integration tests for the FreeWili Finder Rust library

use freewili_finder_rs::*;

#[test]
fn test_usb_device_type_basic() {
    // Test that the enum variants are properly defined
    assert_eq!(UsbDeviceType::Hub, UsbDeviceType::Hub);
    assert_ne!(UsbDeviceType::SerialMain, UsbDeviceType::SerialDisplay);
}

#[test]
fn test_usb_device_display() {
    // Test that Display implementation works
    let usb_device = UsbDevice {
        kind: UsbDeviceType::SerialMain,
        vid: 0x1234,
        pid: 0x5678,
        name: "Test Device".to_string(),
        serial: "TEST123".to_string(),
        location: 0,
        port: Some("/dev/ttyUSB0".to_string()),
        paths: None,
    };

    let display_string = format!("{usb_device}");
    assert_eq!(display_string, "Main: Test Device: /dev/ttyUSB0");
}

#[test]
fn test_usb_device_with_paths() {
    // Test USB device with paths instead of port
    let usb_device = UsbDevice {
        kind: UsbDeviceType::MassStorage,
        vid: 0x1234,
        pid: 0x5678,
        name: "Storage Device".to_string(),
        serial: "STORAGE123".to_string(),
        location: 0,
        port: None,
        paths: Some(vec!["/dev/sda".to_string(), "/dev/sdb".to_string()]),
    };

    let display_string = format!("{usb_device}");
    assert_eq!(
        display_string,
        "Storage: Storage Device: /dev/sda, /dev/sdb"
    );
}

#[test]
fn test_error_conversion() {
    // Test error conversion from FFI error types
    let ffi_error = ffi::_fw_error_t::fw_error_invalid_device as u32;
    let rust_error: FreeWiliError = ffi_error.into();

    match rust_error {
        FreeWiliError::InvalidDevice => {
            // This is expected
        }
        _ => panic!("Unexpected error type: {rust_error:?}"),
    }
}

#[test]
#[ignore] // This test requires actual hardware and the C library
fn test_find_devices() {
    // This test will only work if:
    // 1. The freewili-finder C library is installed
    // 2. There are actual FreeWili devices connected
    // 3. The user has permissions to access USB devices

    match FreeWiliDevice::find_all() {
        Ok(devices) => {
            println!("Found {} devices", devices.len());
            for device in devices {
                println!("Device: {device}");
                assert!(device.is_valid());
            }
        }
        Err(e) => {
            println!("Device discovery failed (expected if no devices connected): {e}",);
            // This is expected if no devices are connected or library not installed
        }
    }
}
