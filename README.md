# FreeWili Finder - Rust Bindings

<div align="center">
  <img src="https://raw.githubusercontent.com/freewili/freewili-finder-rs/master/images/freewili.png" alt="FreeWili" width="400">
</div>

Safe Rust bindings for [freewili-finder](https://github.com/freewili/freewili-finder) discovering and interfacing with FreeWili devices. This library provides a simple, memory-safe API to find FreeWili devices and enumerate their USB components.

## Installation

`cargo add freewili-finder-rs`

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

Run the included example:

```bash
cargo run --example list_all
```

```
Found 3 FreeWili(s)
1. DEFCON25 v72 (USB Composite Device) 29614E37FB9CD753 (Device Type: DefCon2025FwBadge)
        1. Main: DEFCON25 v72 (USB Composite Device): COM91
2. RP2350 Boot (USB Mass Storage Device) A&2037E964&0&0000 (Device Type: UF2)
        1. Storage: RP2350 Boot (USB Mass Storage Device): F:\
3. FreeWili (USB Serial Converter) FW4849 (Device Type: FreeWili)
        1. Hub:  (Generic USB Hub):
        2. Storage: RP2 Boot (USB Composite Device): D:\
        3. Display: DisplayCPU v55 (USB Composite Device): COM46
        4. FPGA: FreeWili (USB Serial Converter): COM51
```

## Error Handling

All functions return `Result<T, FreeWiliError>` with detailed error information:

```rust
match FreeWiliDevice::find_all() {
    Ok(devices) => println!("Found {} devices", devices.len()),
    Err(e) => eprintln!("Error: {}", e),
}
```

## License

MIT License - see [LICENSE](LICENSE) file for details.
