# FreeWili Finder - Rust Bindings

<div align="center">
  <img src="https://raw.githubusercontent.com/freewili/freewili-finder-rs/master/images/freewili.png" alt="FreeWili" width="400">
</div>

[![Crates.io](https://img.shields.io/crates/v/freewili-finder-rs.svg)](https://crates.io/crates/freewili-finder-rs)
[![Documentation](https://docs.rs/freewili-finder-rs/badge.svg)](https://docs.rs/freewili-finder-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Safe Rust bindings for the [freewili-finder](https://github.com/freewili/freewili-finder) C/C++ library for discovering and interfacing with FreeWili devices. This library provides a simple, memory-safe API to find FreeWili devices and enumerate their USB components.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
freewili-finder-rs = "0.4.2"
```

Or use cargo:

```bash
cargo add freewili-finder-rs
```

### Prerequisites

You need build tools for compiling the C++ library:

- CMake 3.20 or later
- C++23 compatible compiler (GCC 11+, Clang 14+, MSVC 2022+)
- Platform dependencies:
    - Linux: libudev-dev
    - macOS: Xcode command line tools
    - Windows: Windows SDK (SetupAPI, Cfgmgr32)

## Quick Start

```rust
use freewili_finder_rs::{FreeWiliDevice, FreeWiliError};

fn main() -> Result<(), FreeWiliError> {
    // Find all connected FreeWili devices
    let devices = FreeWiliDevice::find_all()?;
    
    println!("Found {} FreeWili devices", devices.len());
    
    for (i, device) in devices.iter().enumerate() {
        println!("{}. Found device: {}", i + 1, device);
        
        // Get device information
        let device_type = device.device_type()?;
        let name = device.name()?;
        let serial = device.serial()?;
        let unique_id = device.unique_id()?;
        let standalone = device.standalone()?;
        
        println!("\tType: {:?}", device_type);
        println!("\tName: {}", name);
        println!("\tSerial: {}", serial);
        println!("\tUnique ID: {}", unique_id);
        println!("\tStandalone: {}", standalone);
        
        // Get USB devices
        let usb_devices = device.get_usb_devices()?;
        println!("\tUSB devices ({}):", usb_devices.len());
        
        for (j, usb_device) in usb_devices.iter().enumerate() {
            println!("\t\t{}: {} (VID: {:04X}, PID: {:04X})", 
                j + 1, usb_device.kind_name, usb_device.vid, usb_device.pid);
        }
        
        // Try to get specific USB device types
        if let Ok(main_device) = device.get_main_usb_device() {
            println!("\tMain USB device: {}", main_device.kind_name);
        }
        
        if let Ok(display_device) = device.get_display_usb_device() {
            println!("\tDisplay USB device: {}", display_device.kind_name);
        }
    }
    
    Ok(())
}
```

## Examples

Run the included example to see detailed device information:

```bash
cargo run --example list_all
```

This will output detailed information about all discovered devices:

```
1. Found device: Free-WiLi FW4037
        type: Freewili
        type name: Free-WiLi
        name: Free-WiLi
        serial: FW4037
        unique ID: 259
        standalone: false
        USB devices (4):
                1: Serial Main
                        name: FreeWili MainCPU v73
                        serial: E463A8574B551838
                        VID: 2364 PID: 8276
                        location: 1
                        port chain: [3, 4, 1]
                        port: /dev/ttyACM0
                2: Serial Display
                        name: FreeWili DisplayCPU v55
                        serial: E463A8574B191638
                        VID: 2364 PID: 8277
                        location: 2
                        port chain: [3, 4, 2]
                        port: /dev/ttyACM1
                3: FTDI
                        name: Intrepid FreeWili
                        serial: FW4037
                        VID: 1027 PID: 24596
                        location: 3
                        port chain: [3, 4, 3]
                        port: /dev/ttyUSB0
                4: Hub
                        name: 
                        serial: 
                        VID: 1060 PID: 9491
                        location: 4
                        port chain: [3, 4]
        Main USB device: Serial Main
        Display USB device: Serial Display
        FPGA USB device: FTDI
        HUB USB device: Hub
```

## API Overview

The library provides several key types:

### `FreeWiliDevice`
The main device handle with methods like:
- `find_all()` - Discover all connected FreeWili devices
- `device_type()` - Get the device type (Freewili, Defcon2024Badge, etc.)
- `name()`, `serial()`, `unique_id()` - Get device identification
- `get_usb_devices()` - Get all USB devices associated with this FreeWili device
- `get_main_usb_device()`, `get_display_usb_device()`, etc. - Get specific USB device types

### `USBDevice`
Represents individual USB devices with properties:
- `kind` - USB device type (Hub, Serial, MassStorage, etc.)
- `kind_name` - Human-readable device type name
- `name`, `serial` - Device identification
- `vid`, `pid` - USB Vendor and Product IDs
- `location` - Physical location identifier
- `port_chain` - USB port chain from root hub
- `path`, `port` - Optional system-specific identifiers

### `DeviceType`
Enum representing different FreeWili device types:
- `Freewili` - Standard FreeWili device
- `Defcon2024Badge` - DEFCON 2024 badge
- `Defcon2025FwBadge` - DEFCON 2025 FreeWili badge
- `Uf2` - UF2 bootloader device
- `Winky` - Winky device
- `Unknown` - Unknown or unsupported device

## Error Handling

All functions return `Result<T, FreeWiliError>` with detailed error information:

```rust
use freewili_finder_rs::{FreeWiliDevice, FreeWiliError};

match FreeWiliDevice::find_all() {
    Ok(devices) => {
        println!("Found {} devices", devices.len());
        for device in devices {
            match device.name() {
                Ok(name) => println!("Device: {}", name),
                Err(FreeWiliError::InvalidDevice) => {
                    println!("Device disconnected");
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
    Err(FreeWiliError::InternalError(Some(msg))) => {
        eprintln!("Internal error: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Error Types

- `InvalidParameter` - Invalid parameter passed to function
- `InvalidDevice` - Device handle is invalid (device disconnected)
- `InternalError(Option<String>)` - Internal C library error with optional message
- `MemoryError` - Memory allocation error
- `NoMoreDevices` - No more devices during enumeration
- `None` - Success (internal use)

## Building from Source

To build the library from source, you'll need:

### Prerequisites

- Rust 2024 edition (1.82+)
- CMake 3.20 or later
- C++23 compatible compiler:
  - GCC 11+ on Linux
  - Clang 14+ on macOS
  - MSVC 2022+ on Windows

### Platform-specific Dependencies

**Linux:**
```bash
# Ubuntu/Debian
sudo apt-get install libudev-dev cmake build-essential

# Fedora/RHEL
sudo dnf install libudev-devel cmake gcc-c++

# Arch Linux
sudo pacman -S systemd cmake gcc
```

**macOS:**
```bash
# Install Xcode command line tools
xcode-select --install

# Or install via Homebrew
brew install cmake
```

**Windows:**
- Visual Studio 2022 with C++ tools
- Windows SDK (for SetupAPI and Cfgmgr32)
- CMake (via Visual Studio installer or standalone)

### Building

```bash
git clone https://github.com/freewili/freewili-finder-rs.git
cd freewili-finder-rs
cargo build --release
```

## Testing

Run the test suite:

```bash
cargo test
```

Run the example to verify everything works:

```bash
cargo run --example list_all
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run `cargo test` and `cargo clippy`
6. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [freewili-finder](https://github.com/freewili/freewili-finder) - The underlying C/C++ library
- [FreeWili](https://github.com/freewili) - The main FreeWili project
