# FreeWili Finder - Rust Bindings

Safe Rust bindings for discovering and interfacing with FreeWili devices. This library provides a simple, memory-safe API to find FreeWili devices and enumerate their USB components.

## Features

- **Static Linking**: No runtime dependencies - everything is embedded in your executable
- **Zero Setup**: Just add to Cargo.toml and it works - no DLLs or external libraries needed
- **Cross-Platform**: Supports Windows, macOS, and Linux
- **Device Discovery**: Find all connected FreeWili devices automatically
- **USB Enumeration**: List all USB devices associated with each FreeWili
- **Type Safety**: Strongly-typed device classification system
- **Memory Safe**: Safe Rust wrappers around the underlying C library

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
freewili-finder-rs = "0.2.1"
```

That's it! The library automatically builds and statically links the underlying C++ library - no additional setup required.

### Prerequisites

You need build tools for compiling the C++ library:

- **Windows**: Visual Studio 2022 with C++ tools, CMake
- **Linux**: `build-essential cmake libudev-dev clang`
- **macOS**: Xcode command line tools, CMake

## Quick Start

```rust
use freewili_finder_rs::{FreeWiliDevice, FreeWiliError};

fn main() -> Result<(), FreeWiliError> {
    // Find all connected FreeWili devices
    let devices = FreeWiliDevice::find_all()?;
    
    println!("Found {} FreeWili device(s)", devices.len());
    
    for (i, device) in devices.iter().enumerate() {
        if !device.is_valid() {
            continue;
        }
        
        println!("{}. {}", i + 1, device);
        
        // Get USB devices
        let usb_devices = device.get_usb_devices()?;
        for (j, usb_device) in usb_devices.iter().enumerate() {
            println!("  {}. {}", j + 1, usb_device);
        }
    }
    
    Ok(())
}
```

## Examples

Find serial ports:

```rust
use freewili_finder_rs::{FreeWiliDevice, UsbDeviceType};

fn find_serial_ports() -> Result<Vec<String>, freewili_finder_rs::FreeWiliError> {
    let devices = FreeWiliDevice::find_all()?;
    let mut ports = Vec::new();
    
    for device in devices {
        let usb_devices = device.get_usb_devices()?;
        for usb_device in usb_devices {
            if matches!(usb_device.kind, 
                UsbDeviceType::SerialMain | 
                UsbDeviceType::SerialDisplay |
                UsbDeviceType::Ftdi
            ) {
                if let Some(port) = usb_device.port {
                    ports.push(port);
                }
            }
        }
    }
    
    Ok(ports)
}
```

Run the included example:

```bash
cargo run --example list_all
```

## API Overview

### Main Types

- **`FreeWiliDevice`** - Handle to a FreeWili device
- **`UsbDevice`** - Information about a USB device
- **`UsbDeviceType`** - Enum for device types (SerialMain, SerialDisplay, Ftdi, MassStorage, etc.)

### Key Methods

```rust
// Find all devices
FreeWiliDevice::find_all() -> Result<Vec<FreeWiliDevice>, FreeWiliError>

// Check if device is valid
device.is_valid() -> bool

// Get USB devices
device.get_usb_devices() -> Result<Vec<UsbDevice>, FreeWiliError>

// Get device info
device.get_device_string(string_type) -> Result<String, FreeWiliError>
```

## Static Linking Benefits

✅ **No runtime dependencies** - Single executable with everything embedded  
✅ **Easy deployment** - Just ship your executable  
✅ **No DLL hell** - No missing library issues  
✅ **Works everywhere** - No need to install additional packages  

## Error Handling

All functions return `Result<T, FreeWiliError>` with detailed error information:

```rust
match FreeWiliDevice::find_all() {
    Ok(devices) => println!("Found {} devices", devices.len()),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Platform Support

| Platform | Status | System Libraries |
|----------|--------|------------------|
| Windows  | ✅     | SetupAPI, Cfgmgr32 |
| Linux    | ✅     | libudev |
| macOS    | ✅     | IOKit framework |

## License

MIT License - see [LICENSE](LICENSE) file for details.
