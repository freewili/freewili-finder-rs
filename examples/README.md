# FreeWili Finder Examples

This directory contains examples demonstrating how to use the FreeWili Finder Rust library.

## Available Examples

### `list_all.rs`
A comprehensive example that demonstrates:
- Finding all connected FreeWili devices
- Checking device validity
- Getting basic device information (name and serial using Display trait)
- Enumerating USB devices for each FreeWili device
- Displaying USB device information (type, name, ports, and paths)
- Error handling

Run with:
```bash
cargo run --example list_all
```

## Running Examples

To run any example:

```bash
cargo run --example <example_name>
```

For example:
```bash
cargo run --example list_all
```

## Prerequisites

- A FreeWili device connected to your system
- The device should be powered on and properly connected via USB
- Appropriate permissions to access USB devices (on Linux, you may need to run as root or configure udev rules)

## Expected Output

When a FreeWili device is connected, you should see output similar to:

```
Found 1 FreeWili(s)
1. Intrepid FreeWili FW5419
        1. Storage: Raspberry Pi RP2 Boot: /run/media/user/RPI-RP2
        2. Display: FreeWili DisplayCPU v47: /dev/ttyACM0
        3. FPGA: Intrepid FreeWili: /dev/ttyUSB0
```

If no devices are found, check:
1. Device is powered on
2. USB cable is properly connected
3. Device drivers are installed (if required)
4. You have proper permissions to access the device

## Features Demonstrated

The examples showcase:
- **Device Discovery**: Finding all connected FreeWili devices
- **Device Information**: Getting device name, serial number, and validity status
- **USB Device Enumeration**: Listing all USB devices associated with each FreeWili device
- **Device Type Recognition**: Identifying different types of USB devices (Storage, Display, FPGA, etc.)
- **Path and Port Information**: Displaying device paths for storage devices and serial ports for communication devices
- **Error Handling**: Proper error handling throughout the discovery process
- **Display Formatting**: Clean, readable output using custom Display trait implementations
