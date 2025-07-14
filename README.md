# FreeWili Finder - Rust Bindings

A Rust library providing safe bindings for the [FreeWili Finder](https://github.com/freewili/freewili-finder) C/C++ library. This library automatically downloads, builds, and generates bindings for the freewili-finder library, making it easy to discover and interface with FreeWili devices from Rust applications.

## Features

- **Automatic Build**: Automatically clones and builds the freewili-finder library using CMake
- **Generated Bindings**: Uses bindgen to automatically generate Rust bindings from C headers
- **Device Discovery**: Find all connected FreeWili devices automatically
- **USB Enumeration**: List all USB devices associated with each FreeWili
- **Type Classification**: Identify device types (Serial ports, Mass storage, FTDI/FPGA, etc.)
- **Device Information**: Access VID/PID, serial numbers, port paths, and more
- **Safe Rust API**: Memory-safe Rust wrappers around the C API
- **Cross-Platform**: Supports Windows, macOS, and Linux

## Installation

### Prerequisites

The library will automatically build the freewili-finder C/C++ library, but you need to have the build tools installed:

#### On Windows

1. Install Visual Studio 2022 with C++ development tools
2. Install CMake: https://cmake.org/download/
3. Install Git: https://git-scm.com/downloads
4. Make sure both CMake and Git are in your PATH

#### On Linux

```bash
sudo apt-get update
sudo apt-get install build-essential cmake git libudev-dev pkg-config clang libclang-dev
```

#### On macOS

```bash
# Install Xcode command line tools
xcode-select --install

# Install CMake via Homebrew
brew install cmake git llvm
```

### Add to your Rust project

Add this to your `Cargo.toml`:

```toml
[dependencies]
freewili-finder-rs = "0.1.0"
```

That's it! The library will automatically:
1. Clone the freewili-finder repository
2. Build it with CMake
3. Generate Rust bindings with bindgen
4. Link everything together

## Usage

### Basic Example

```rust
use freewili_finder_rs::{FreeWiliDevice, FreeWiliError};

fn main() -> Result<(), FreeWiliError> {
    // Find all connected FreeWili devices
    let devices = FreeWiliDevice::find_all()?;
    
    if devices.is_empty() {
        println!("No FreeWili devices found.");
        return Ok(());
    }
    
    println!("Found {} FreeWili device(s):", devices.len());
    
    for (i, device) in devices.iter().enumerate() {
        println!("{}. {}", i + 1, device); // Uses Display trait
        
        // Check if device is valid
        if !device.is_valid() {
            println!("  Device is invalid, skipping...");
            continue;
        }
        
        // Get USB devices
        match device.get_usb_devices() {
            Ok(usb_devices) => {
                for (j, usb_device) in usb_devices.iter().enumerate() {
                    println!("  {}. {}", j + 1, usb_device); // Uses Display trait
                }
            }
            Err(e) => {
                println!("  Error getting USB devices: {}", e);
            }
        }
    }
    
    Ok(())
}
```

### Working with Serial Ports

```rust
use freewili_finder_rs::{FreeWiliDevice, FreeWiliError, UsbDeviceType};

fn find_serial_ports() -> Result<Vec<String>, FreeWiliError> {
    let devices = FreeWiliDevice::find_all()?;
    let mut ports = Vec::new();
    
    for device in devices {
        if !device.is_valid() {
            continue;
        }
        
        let usb_devices = device.get_usb_devices()?;
        for usb_device in usb_devices {
            // Check for serial device types
            if matches!(usb_device.kind, 
                UsbDeviceType::Serial | 
                UsbDeviceType::SerialMain | 
                UsbDeviceType::SerialDisplay
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

### Device Type Filtering

```rust
use freewili_finder_rs::{FreeWiliDevice, FreeWiliError, UsbDeviceType};

fn categorize_devices() -> Result<(), FreeWiliError> {
    let devices = FreeWiliDevice::find_all()?;
    
    for device in devices {
        println!("Device: {}", device); // Uses Display trait
        
        if !device.is_valid() {
            println!("  Device is invalid, skipping...");
            continue;
        }
        
        let usb_devices = device.get_usb_devices()?;
        
        // Group devices by type
        let mut type_counts = std::collections::HashMap::new();
        for usb_device in &usb_devices {
            *type_counts.entry(usb_device.kind).or_insert(0) += 1;
        }
        
        // Display counts for each type
        for (device_type, count) in type_counts {
            let type_name = match device_type {
                UsbDeviceType::SerialMain => "Main CPU",
                UsbDeviceType::SerialDisplay => "Display CPU",
                UsbDeviceType::Ftdi => "FPGA",
                UsbDeviceType::MassStorage => "Storage",
                UsbDeviceType::Serial => "Serial",
                UsbDeviceType::Hub => "Hub",
                UsbDeviceType::Esp32 => "ESP32",
                UsbDeviceType::Other => "Other",
                UsbDeviceType::_MaxValue => "Unknown",
            };
            println!("  {}: {} device(s)", type_name, count);
        }
    }
    
    Ok(())
}
```

## API Reference

### Types

#### `UsbDeviceType`
Enumeration of USB device types:
- `Hub` - USB Hub (parent device)
- `Serial` - Serial Port (general)
- `SerialMain` - MainCPU Serial Port
- `SerialDisplay` - DisplayCPU Serial Port
- `MassStorage` - Mass Storage Device
- `Esp32` - ESP32 USB (JTAG/RTT)
- `Ftdi` - FTDI/FPGA Device
- `Other` - Other USB device

#### `FreeWiliDevice`
The main device handle:
```rust
pub struct FreeWiliDevice {
    pub handle: *mut fw_freewili_device_t,
}

impl FreeWiliDevice {
    pub fn find_all() -> Result<Vec<FreeWiliDevice>, FreeWiliError>;
    pub fn is_valid(&self) -> bool;
    pub fn get_device_string(&self, string_type: fw_stringtype_t) -> Result<String, FreeWiliError>;
    pub fn get_usb_devices(&self) -> Result<Vec<UsbDevice>, FreeWiliError>;
}

impl fmt::Display for FreeWiliDevice {
    // Displays: "{name} {serial}"
}
```

#### `UsbDevice`
Information about a USB device:
```rust
pub struct UsbDevice {
    pub kind: UsbDeviceType,
    pub vid: u16,
    pub pid: u16,
    pub name: String,
    pub serial: String,
    pub location: u32,
    pub port: Option<String>,           // For serial devices
    pub paths: Option<Vec<String>>,     // For mass storage devices
}

impl fmt::Display for UsbDevice {
    // Displays: "{type}: {name}: {port/paths}"
}
```

### Functions

#### `FreeWiliDevice::find_all() -> Result<Vec<FreeWiliDevice>, FreeWiliError>`
Find all connected FreeWili devices.

#### `FreeWiliDevice::is_valid(&self) -> bool`
Check if a device handle is valid.

#### `FreeWiliDevice::get_device_string(&self, string_type) -> Result<String, FreeWiliError>`
Get device information strings (name, serial, path, port).

#### `FreeWiliDevice::get_usb_devices(&self) -> Result<Vec<UsbDevice>, FreeWiliError>`
Get all USB devices associated with this FreeWili device.

### String Types

Available string types for `get_device_string()`:
- `fw_stringtype_name` - Device name
- `fw_stringtype_serial` - Device serial number
- `fw_stringtype_path` - Device path
- `fw_stringtype_port` - Device port

## Examples

Run the included examples:

```bash
# List all devices and their USB components
cargo run --example list_all
```

See the [examples/README.md](examples/README.md) for more detailed information about available examples.

## Building

To build the library:

```bash
cargo build
```

**Note**: The first build will take longer as it downloads and compiles the freewili-finder C++ library. Subsequent builds will be much faster.

To run tests:

```bash
cargo test
```

## Error Handling

The library uses the `FreeWiliError` enum for error handling:

```rust
#[derive(Error, Debug)]
pub enum FreeWiliError {
    #[error("Discovery failed: {0}")]
    DiscoveryFailed(String),
    #[error("Invalid Parameter")]
    InvalidParameter,
    #[error("Invalid device handle")]
    InvalidDevice,
    #[error("Internal error")]
    InternalError,
    #[error("Memory error")]
    MemoryError,
    #[error("No more devices found")]
    NoMoreDevices,
    #[error("None")]
    None,
}
```

## Platform Support

| Platform | Status | Dependencies |
|----------|--------|-------------|
| Linux    | ✅ Supported | libudev |
| macOS    | ✅ Supported | IOKit framework |
| Windows  | ✅ Supported | Windows SDK (SetupAPI, Cfgmgr32) |

## CI/CD Setup

This repository includes comprehensive GitHub Actions workflows for continuous integration and deployment.

### Required Secrets

For automated publishing to crates.io:
```
CRATES_TOKEN: Your crates.io API token
```

### Optional Secrets

For code coverage reporting (optional):
```
CODECOV_TOKEN: Your Codecov repository token
```

If you don't set up Codecov, coverage reports will still be generated and uploaded as GitHub artifacts.

### Setting up Codecov (Optional)

1. Go to [codecov.io](https://codecov.io) and sign up with your GitHub account
2. Add your repository to Codecov
3. Get your repository token from the Codecov dashboard
4. Add `CODECOV_TOKEN` secret to your GitHub repository settings

If you don't set up Codecov:
- Coverage reports will still be generated
- Reports will be uploaded as GitHub artifacts
- You can download and view them from the Actions tab

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

The underlying freewili-finder library is also licensed under the MIT License.

## Troubleshooting

### Common Issues

1. **Library not found errors**: Make sure the freewili-finder C library is properly installed and in your system's library path.

2. **Permission denied**: On Linux/macOS, you may need to run with elevated privileges or add your user to the appropriate groups to access USB devices.

3. **No devices found**: Ensure that:
   - FreeWili devices are properly connected
   - Device drivers are installed
   - The devices are recognized by the operating system

### Getting Help

- Check the [freewili-finder documentation](https://github.com/freewili/freewili-finder)
- Open an issue on this repository
- Review the example code for common usage patterns

## Dependencies

This library automatically handles its dependencies by:

1. **Downloading**: Automatically clones the freewili-finder repository
2. **Building**: Compiles the C++ library using CMake
3. **Binding**: Generates Rust bindings using bindgen

### Build Dependencies
- [thiserror](https://crates.io/crates/thiserror) - For error handling
- [bindgen](https://crates.io/crates/bindgen) - For generating Rust bindings from C headers (build-time)
- [cmake](https://crates.io/crates/cmake) - For building the C++ library (build-time)
- [cc](https://crates.io/crates/cc) - For C compilation support (build-time)

### System Dependencies
- **CMake** - For building the freewili-finder library
- **Git** - For cloning the repository
- **C++ Compiler** - Visual Studio on Windows, GCC/Clang on Linux/macOS
- **Platform Libraries**:
  - Windows: SetupAPI, Cfgmgr32
  - Linux: libudev
  - macOS: IOKit, Foundation
