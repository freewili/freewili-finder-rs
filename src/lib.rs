//! # FreeWili Finder - Rust Bindings
//!
//! This library provides safe Rust bindings for the FreeWili Finder C/C++ library,
//! making it easy to discover and interface with FreeWili devices from Rust applications.
//!
//! ## Features
//!
//! - **Device Discovery**: Find all connected FreeWili devices automatically
//! - **USB Enumeration**: List all USB devices associated with each FreeWili device
//! - **Type Classification**: Identify device types (Serial ports, Mass storage, FTDI/FPGA, etc.)
//! - **Device Information**: Access VID/PID, serial numbers, port paths, and more
//! - **Safe Rust API**: Memory-safe Rust wrappers around the C API
//! - **Cross-Platform**: Supports Windows, macOS, and Linux
//!
//! ## Quick Start
//!
//! ```rust
//! use freewili_finder_rs::{FreeWiliDevice, FreeWiliError};
//!
//! fn main() -> Result<(), FreeWiliError> {
//!     // Find all connected FreeWili devices
//!     let devices = FreeWiliDevice::find_all()?;
//!     
//!     for device in devices {
//!         println!("Found: {}", device); // Uses Display trait
//!         
//!         if device.is_valid() {
//!             let usb_devices = device.get_usb_devices()?;
//!             for usb_device in usb_devices {
//!                 println!("  {}", usb_device); // Type: Name: Port/Path
//!             }
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! All functions return `Result<T, FreeWiliError>` for proper error handling.
//! The library provides detailed error messages for common failure scenarios.

/// Foreign Function Interface (FFI) bindings to the FreeWili Finder C library.
///
/// This module contains the automatically generated bindings from the C header files.
/// These bindings are used internally by the safe Rust wrapper functions.
///
/// # Warning
///
/// The functions in this module are unsafe and should not be used directly.
/// Use the safe wrapper functions in the parent module instead.
pub mod ffi;

use ffi::fw_error_t;
use ffi::fw_freewili_device_t;
use std::ffi::{c_char, CStr};
use std::fmt;
use std::ptr;
use thiserror::Error;

// Re-export the enum variants as constants for easier use
/// String type constants for use with [`FreeWiliDevice::get_device_string`].
///
/// These constants specify what type of string information to retrieve from a device.
///
/// # Available String Types
///
/// - [`fw_stringtype_name`] - Device name (e.g., "Intrepid FreeWili")
/// - [`fw_stringtype_serial`] - Device serial number (e.g., "FW5419")
/// - [`fw_stringtype_path`] - Device path (for storage devices)
/// - [`fw_stringtype_port`] - Device port (for serial devices like "/dev/ttyUSB0")
/// - [`fw_stringtype_raw`] - Raw system path
/// - [`fw_stringtype_type`] - Device type name
pub use ffi::_fw_stringtype_t::*;

/// Integer type constants for internal use with USB device enumeration.
///
/// These constants specify what type of integer information to retrieve from a USB device.
///
/// # Available Integer Types
///
/// - [`fw_inttype_vid`] - USB Vendor ID
/// - [`fw_inttype_pid`] - USB Product ID  
/// - [`fw_inttype_location`] - USB location identifier
pub use ffi::_fw_inttype_t::*;

/// Error types that can occur when using the FreeWili Finder library.
///
/// This enum provides detailed error information for various failure scenarios
/// that can occur during device discovery, USB enumeration, and device communication.
///
/// # Examples
///
/// ```rust
/// use freewili_finder_rs::{FreeWiliDevice, FreeWiliError};
///
/// match FreeWiliDevice::find_all() {
///     Ok(devices) => println!("Found {} devices", devices.len()),
///     Err(FreeWiliError::DiscoveryFailed(msg)) => {
///         eprintln!("Discovery failed: {}", msg);
///     }
///     Err(e) => eprintln!("Other error: {}", e),
/// }
/// ```
#[derive(Error, Debug)]
pub enum FreeWiliError {
    /// Device discovery failed with a detailed error message
    #[error("Discovery failed: {0}")]
    DiscoveryFailed(String),
    /// Invalid parameter passed to a function
    #[error("Invalid Parameter")]
    InvalidParameter,
    /// Invalid device handle (device may have been disconnected)
    #[error("Invalid device handle")]
    InvalidDevice,
    /// Internal error in the underlying C library
    #[error("Internal error")]
    InternalError,
    /// Memory allocation error
    #[error("Memory error")]
    MemoryError,
    /// No more devices found during enumeration
    #[error("No more devices found")]
    NoMoreDevices,
    /// Success or no error (used internally)
    #[error("None")]
    None,
}

impl From<ffi::_fw_error_t> for FreeWiliError {
    fn from(error: ffi::_fw_error_t) -> Self {
        match error {
            ffi::_fw_error_t::fw_error_success => FreeWiliError::None,
            ffi::_fw_error_t::fw_error_invalid_parameter => FreeWiliError::InvalidParameter,
            ffi::_fw_error_t::fw_error_invalid_device => FreeWiliError::InvalidDevice,
            ffi::_fw_error_t::fw_error_internal_error => FreeWiliError::InternalError,
            ffi::_fw_error_t::fw_error_memory => FreeWiliError::MemoryError,
            ffi::_fw_error_t::fw_error_no_more_devices => FreeWiliError::NoMoreDevices,
            ffi::_fw_error_t::fw_error_none => FreeWiliError::None,
            ffi::_fw_error_t::fw_error__maxvalue => FreeWiliError::InternalError,
        }
    }
}

impl From<fw_error_t> for FreeWiliError {
    fn from(error_code: fw_error_t) -> Self {
        match error_code {
            x if x == ffi::_fw_error_t::fw_error_success as u32 => FreeWiliError::None,
            x if x == ffi::_fw_error_t::fw_error_invalid_parameter as u32 => {
                FreeWiliError::InvalidParameter
            }
            x if x == ffi::_fw_error_t::fw_error_invalid_device as u32 => {
                FreeWiliError::InvalidDevice
            }
            x if x == ffi::_fw_error_t::fw_error_internal_error as u32 => {
                FreeWiliError::InternalError
            }
            x if x == ffi::_fw_error_t::fw_error_memory as u32 => FreeWiliError::MemoryError,
            x if x == ffi::_fw_error_t::fw_error_no_more_devices as u32 => {
                FreeWiliError::NoMoreDevices
            }
            x if x == ffi::_fw_error_t::fw_error_none as u32 => FreeWiliError::None,
            _ => FreeWiliError::InternalError,
        }
    }
}

/// Type of USB device connected to a FreeWili device.
///
/// This enum categorizes the different types of USB devices that can be
/// associated with a FreeWili device, helping to identify their purpose
/// and connection interface.
///
/// # Examples
///
/// ```rust
/// use freewili_finder_rs::{UsbDeviceType, UsbDevice};
///
/// let usb_device = UsbDevice {
///     kind: UsbDeviceType::SerialMain,
///     vid: 0x1234,
///     pid: 0x5678,
///     name: "Test Device".to_string(),
///     serial: "TEST123".to_string(),
///     location: 0,
///     port: Some("/dev/ttyUSB0".to_string()),
///     paths: None,
/// };
///
/// match usb_device.kind {
///     UsbDeviceType::SerialMain => println!("Main CPU serial port"),
///     UsbDeviceType::SerialDisplay => println!("Display CPU serial port"),
///     UsbDeviceType::Ftdi => println!("FPGA device"),
///     UsbDeviceType::MassStorage => println!("Storage device"),
///     _ => println!("Other device type"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbDeviceType {
    /// USB Hub (parent device)
    Hub,
    /// Generic Serial Port device
    Serial,
    /// Main CPU Serial Port
    SerialMain,
    /// Display CPU Serial Port
    SerialDisplay,
    /// Mass Storage Device (e.g., SD card, flash drive)
    MassStorage,
    /// ESP32 USB device (JTAG/RTT)
    Esp32,
    /// FTDI device (typically connected to FPGA)
    Ftdi,
    /// Other/Unknown USB device
    Other,
    /// Maximum value marker (internal use)
    _MaxValue,
}

impl From<ffi::_fw_usbdevicetype_t> for UsbDeviceType {
    fn from(device_type: ffi::_fw_usbdevicetype_t) -> Self {
        match device_type {
            ffi::_fw_usbdevicetype_t::fw_usbdevicetype_hub => UsbDeviceType::Hub,
            ffi::_fw_usbdevicetype_t::fw_usbdevicetype_serial => UsbDeviceType::Serial,
            ffi::_fw_usbdevicetype_t::fw_usbdevicetype_serialmain => UsbDeviceType::SerialMain,
            ffi::_fw_usbdevicetype_t::fw_usbdevicetype_serialdisplay => {
                UsbDeviceType::SerialDisplay
            }
            ffi::_fw_usbdevicetype_t::fw_usbdevicetype_massstorage => UsbDeviceType::MassStorage,
            ffi::_fw_usbdevicetype_t::fw_usbdevicetype_esp32 => UsbDeviceType::Esp32,
            ffi::_fw_usbdevicetype_t::fw_usbdevicetype_ftdi => UsbDeviceType::Ftdi,
            ffi::_fw_usbdevicetype_t::fw_usbdevicetype_other => UsbDeviceType::Other,
            ffi::_fw_usbdevicetype_t::fw_usbdevicetype__maxvalue => UsbDeviceType::_MaxValue,
        }
    }
}

/// Information about a USB device connected to a FreeWili device.
///
/// This structure contains comprehensive information about a USB device,
/// including its type, identifiers, name, and connection details.
///
/// # Fields
///
/// - `kind`: The type of USB device (Serial, Storage, FPGA, etc.)
/// - `vid`: USB Vendor ID
/// - `pid`: USB Product ID  
/// - `name`: Human-readable device name
/// - `serial`: Device serial number
/// - `location`: USB location identifier
/// - `port`: Serial port path (for serial devices)
/// - `paths`: File system paths (for storage devices)
///
/// # Examples
///
/// ```rust
/// use freewili_finder_rs::{FreeWiliDevice, UsbDeviceType};
///
/// let devices = FreeWiliDevice::find_all()?;
/// for device in devices {
///     let usb_devices = device.get_usb_devices()?;
///     for usb_device in usb_devices {
///         println!("Device: {} (VID: {:04x}, PID: {:04x})",
///                  usb_device.name, usb_device.vid, usb_device.pid);
///                  
///         if usb_device.kind == UsbDeviceType::SerialMain {
///             if let Some(port) = &usb_device.port {
///                 println!("  Serial port: {}", port);
///             }
///         }
///     }
/// }
/// # Ok::<(), freewili_finder_rs::FreeWiliError>(())
/// ```
#[derive(Debug, Clone)]
pub struct UsbDevice {
    /// The type of USB device
    pub kind: UsbDeviceType,
    /// USB Vendor ID
    pub vid: u16,
    /// USB Product ID
    pub pid: u16,
    /// Human-readable device name
    pub name: String,
    /// Device serial number
    pub serial: String,
    /// USB location identifier
    pub location: u32,
    /// Serial port path (for serial devices like /dev/ttyUSB0, COM1)
    pub port: Option<String>,
    /// File system paths (for storage devices, may contain multiple mount points)
    pub paths: Option<Vec<String>>,
}

impl fmt::Display for UsbDevice {
    /// Format the USB device for display.
    ///
    /// This implementation displays the device in the format:
    /// "{type}: {name}: {port/paths}"
    ///
    /// Where:
    /// - `type` is a human-readable device type (Main, Display, FPGA, Storage, etc.)
    /// - `name` is the device name
    /// - `port/paths` shows the connection point (serial port for serial devices,
    ///   file paths for storage devices)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use freewili_finder_rs::FreeWiliDevice;
    ///
    /// let devices = FreeWiliDevice::find_all()?;
    /// for device in devices {
    ///     let usb_devices = device.get_usb_devices()?;
    ///     for usb_device in usb_devices {
    ///         println!("{}", usb_device);
    ///         // Output examples:
    ///         // "Display: FreeWili DisplayCPU v47: /dev/ttyACM0"
    ///         // "Storage: Raspberry Pi RP2 Boot: /run/media/user/RPI-RP2"
    ///         // "FPGA: Intrepid FreeWili: /dev/ttyUSB0"
    ///     }
    /// }
    /// # Ok::<(), freewili_finder_rs::FreeWiliError>(())
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Get the device type name
        let type_name = match self.kind {
            UsbDeviceType::SerialMain => "Main",
            UsbDeviceType::SerialDisplay => "Display",
            UsbDeviceType::Ftdi => "FPGA",
            UsbDeviceType::Serial => "Serial",
            UsbDeviceType::Hub => "Hub",
            UsbDeviceType::MassStorage => "Storage",
            UsbDeviceType::Esp32 => "ESP32",
            UsbDeviceType::Other => "Other",
            UsbDeviceType::_MaxValue => "Unknown",
        };

        // Format: "Type: Device Name: port/path"
        write!(f, "{}: {}", type_name, self.name)?;

        if let Some(port) = &self.port {
            write!(f, ": {port}")?;
        } else if let Some(paths) = &self.paths {
            write!(f, ": {}", paths.join(", "))?;
        }

        Ok(())
    }
}

/// A FreeWili device handle.
///
/// This structure represents a connection to a FreeWili device and provides
/// methods to query device information and enumerate associated USB devices.
///
/// # Safety
///
/// The handle is automatically managed - it will be properly freed when the
/// `FreeWiliDevice` is dropped. All unsafe operations are wrapped in safe
/// Rust methods.
///
/// # Examples
///
/// ```rust
/// use freewili_finder_rs::{FreeWiliDevice, fw_stringtype_name, fw_stringtype_serial};
///
/// // Find all devices
/// let devices = FreeWiliDevice::find_all()?;
///
/// for device in devices {
///     if device.is_valid() {
///         // Get device information
///         let name = device.get_device_string(fw_stringtype_name)?;
///         let serial = device.get_device_string(fw_stringtype_serial)?;
///         println!("Device: {} ({})", name, serial);
///         
///         // Enumerate USB devices
///         let usb_devices = device.get_usb_devices()?;
///         println!("  USB devices: {}", usb_devices.len());
///     }
/// }
/// # Ok::<(), freewili_finder_rs::FreeWiliError>(())
/// ```
#[derive(Debug, Clone)]
pub struct FreeWiliDevice {
    /// Raw handle to the C library device structure
    pub handle: *mut fw_freewili_device_t,
}

impl Default for FreeWiliDevice {
    fn default() -> Self {
        FreeWiliDevice {
            handle: ptr::null_mut(),
        }
    }
}

impl Drop for FreeWiliDevice {
    fn drop(&mut self) {
        let _res: ffi::fw_error_t = unsafe { ffi::fw_device_free(&mut self.handle, 1) };
        if _res != ffi::_fw_error_t::fw_error_success as u32 {
            eprintln!("Failed to free FreeWili device handle: {_res:?}");
        }
        self.handle = ptr::null_mut();
    }
}

impl FreeWiliDevice {
    /// Find all connected FreeWili devices.
    ///
    /// This function scans the system for all connected FreeWili devices and
    /// returns a vector of device handles. Each handle can then be used to
    /// query device information and enumerate USB devices.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<FreeWiliDevice>)`: A vector of found devices (may be empty)
    /// - `Err(FreeWiliError)`: If device discovery fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use freewili_finder_rs::FreeWiliDevice;
    ///
    /// match FreeWiliDevice::find_all() {
    ///     Ok(devices) => {
    ///         println!("Found {} FreeWili devices", devices.len());
    ///         for device in devices {
    ///             println!("Device: {}", device);
    ///         }
    ///     }
    ///     Err(e) => eprintln!("Failed to find devices: {}", e),
    /// }
    /// ```
    pub fn find_all() -> Result<Vec<FreeWiliDevice>, FreeWiliError> {
        const MAX_DEVICE_COUNT: u32 = 255;
        let mut device_count: u32 = MAX_DEVICE_COUNT;
        let mut devices: [*mut fw_freewili_device_t; MAX_DEVICE_COUNT as usize] =
            [std::ptr::null_mut(); MAX_DEVICE_COUNT as usize];
        let mut error_msg = vec![0u8; 1024];
        let mut error_size: u32 = error_msg.len() as u32;

        let _res: fw_error_t = unsafe {
            ffi::fw_device_find_all(
                devices.as_mut_ptr(),
                &mut device_count,
                error_msg.as_mut_ptr() as *mut i8,
                &mut error_size,
            )
        };
        if _res != ffi::_fw_error_t::fw_error_success as u32 {
            let error_str = unsafe { CStr::from_ptr(error_msg.as_ptr() as *const c_char) }
                .to_string_lossy()
                .into_owned();
            return Err(FreeWiliError::DiscoveryFailed(error_str));
        }
        let mut device_handles = Vec::with_capacity(device_count as usize);
        for i in 0..device_count {
            device_handles.push(FreeWiliDevice {
                handle: devices[i as usize],
            });
        }
        Ok(device_handles)
    }

    /// Check if the device handle is valid.
    ///
    /// A device handle may become invalid if the device is disconnected
    /// or if there was an error during device discovery.
    ///
    /// # Returns
    ///
    /// `true` if the device handle is valid and can be used, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use freewili_finder_rs::FreeWiliDevice;
    ///
    /// let devices = FreeWiliDevice::find_all()?;
    /// for device in devices {
    ///     if device.is_valid() {
    ///         println!("Device is valid: {}", device);
    ///     } else {
    ///         println!("Device is invalid, skipping...");
    ///     }
    /// }
    /// # Ok::<(), freewili_finder_rs::FreeWiliError>(())
    /// ```
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::fw_device_is_valid(self.handle) }
    }

    /// Get a string value from the device.
    ///
    /// This method retrieves various string properties from the device,
    /// such as the device name, serial number, path, or port information.
    ///
    /// # Arguments
    ///
    /// * `string_type` - The type of string to retrieve (use constants like
    ///   `fw_stringtype_name`, `fw_stringtype_serial`, etc.)
    ///
    /// # Returns
    ///
    /// - `Ok(String)`: The requested string value
    /// - `Err(FreeWiliError)`: If the string cannot be retrieved
    ///
    /// # Examples
    ///
    /// ```rust
    /// use freewili_finder_rs::{FreeWiliDevice, fw_stringtype_name, fw_stringtype_serial};
    ///
    /// let devices = FreeWiliDevice::find_all()?;
    /// for device in devices {
    ///     if device.is_valid() {
    ///         let name = device.get_device_string(fw_stringtype_name)?;
    ///         let serial = device.get_device_string(fw_stringtype_serial)?;
    ///         println!("Device: {} (Serial: {})", name, serial);
    ///     }
    /// }
    /// # Ok::<(), freewili_finder_rs::FreeWiliError>(())
    /// ```
    pub fn get_device_string(
        &self,
        string_type: ffi::_fw_stringtype_t,
    ) -> Result<String, FreeWiliError> {
        let mut buffer = vec![0u8; 1024];
        let buffer_size = buffer.len() as u32;

        let res: fw_error_t = unsafe {
            ffi::fw_device_get_str(
                self.handle,
                string_type as u32,
                buffer.as_mut_ptr() as *mut c_char,
                buffer_size,
            )
        };
        if res != ffi::_fw_error_t::fw_error_success as u32 {
            return Err(res.into());
        }

        let cstr = unsafe { CStr::from_ptr(buffer.as_ptr() as *const c_char) };
        Ok(cstr.to_string_lossy().into_owned())
    }

    /// Get all USB devices associated with this FreeWili device.
    ///
    /// This method enumerates all USB devices connected to or associated with
    /// the FreeWili device, including serial ports, storage devices, and
    /// other USB interfaces.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<UsbDevice>)`: A vector of USB devices (may be empty)
    /// - `Err(FreeWiliError)`: If USB enumeration fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use freewili_finder_rs::{FreeWiliDevice, UsbDeviceType};
    ///
    /// let devices = FreeWiliDevice::find_all()?;
    /// for device in devices {
    ///     if device.is_valid() {
    ///         let usb_devices = device.get_usb_devices()?;
    ///         println!("Found {} USB devices:", usb_devices.len());
    ///         
    ///         for usb_device in usb_devices {
    ///             match usb_device.kind {
    ///                 UsbDeviceType::SerialMain => {
    ///                     println!("  Main CPU: {}", usb_device.name);
    ///                     if let Some(port) = &usb_device.port {
    ///                         println!("    Port: {}", port);
    ///                     }
    ///                 }
    ///                 UsbDeviceType::MassStorage => {
    ///                     println!("  Storage: {}", usb_device.name);
    ///                     if let Some(paths) = &usb_device.paths {
    ///                         println!("    Paths: {}", paths.join(", "));
    ///                     }
    ///                 }
    ///                 _ => println!("  Other: {}", usb_device),
    ///             }
    ///         }
    ///     }
    /// }
    /// # Ok::<(), freewili_finder_rs::FreeWiliError>(())
    /// ```
    pub fn get_usb_devices(&self) -> Result<Vec<UsbDevice>, FreeWiliError> {
        let mut usb_devices = Vec::new();

        // Begin USB device enumeration
        let res = unsafe { ffi::fw_usb_device_begin(self.handle) };
        if res != ffi::_fw_error_t::fw_error_success as u32 {
            return Err(res.into());
        }

        // Loop through all USB devices
        loop {
            // Try to get the current USB device information
            match unsafe { self.convert_current_usb_device() } {
                Ok(usb_device) => usb_devices.push(usb_device),
                Err(FreeWiliError::NoMoreDevices) => break, // Normal end of enumeration
                Err(e) => {
                    // For other errors, we'll skip this device and continue
                    eprintln!("Warning: Failed to convert USB device: {e}");
                }
            }

            // Move to the next USB device
            let res = unsafe { ffi::fw_usb_device_next(self.handle) };
            if res == ffi::_fw_error_t::fw_error_no_more_devices as u32 {
                break; // No more devices
            }
            if res != ffi::_fw_error_t::fw_error_success as u32 {
                return Err(res.into());
            }
        }

        Ok(usb_devices)
    }

    /// Helper method to convert the current USB device during enumeration.
    ///
    /// This method is used internally during USB device enumeration to
    /// convert the current device information from the C library into
    /// a Rust `UsbDevice` structure.
    ///
    /// # Safety
    ///
    /// This method is marked as unsafe because it calls unsafe FFI functions,
    /// but it is used internally and all safety requirements are met.
    ///
    /// # Returns
    ///
    /// - `Ok(UsbDevice)`: Successfully converted device information
    /// - `Err(FreeWiliError)`: If device information cannot be retrieved
    unsafe fn convert_current_usb_device(&self) -> Result<UsbDevice, FreeWiliError> {
        // Get USB device type
        let mut device_type: u32 = 0;
        let res = unsafe { ffi::fw_usb_device_get_type(self.handle, &mut device_type) };
        if res != ffi::_fw_error_t::fw_error_success as u32 {
            return Err(res.into());
        }

        // Get device strings
        let name = self.get_usb_device_string(fw_stringtype_name)?;
        let serial = self
            .get_usb_device_string(fw_stringtype_serial)
            .unwrap_or_default();
        let port = self.get_usb_device_string(fw_stringtype_port).ok();

        // Get device paths - collect both path and raw path if available
        let mut paths = Vec::new();
        if let Ok(path) = self.get_usb_device_string(fw_stringtype_path) {
            if !path.is_empty() {
                paths.push(path);
            }
        } else if let Ok(raw_path) = self.get_usb_device_string(fw_stringtype_raw) {
            if !raw_path.is_empty() && !paths.contains(&raw_path) {
                paths.push(raw_path);
            }
        }
        let paths = if paths.is_empty() { None } else { Some(paths) };

        // Get device integers
        let vid = self.get_usb_device_int(fw_inttype_vid)? as u16;
        let pid = self.get_usb_device_int(fw_inttype_pid)? as u16;
        let location = self.get_usb_device_int(fw_inttype_location)?;

        // Convert device type
        let kind = match device_type {
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_hub as u32 => UsbDeviceType::Hub,
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_serial as u32 => {
                UsbDeviceType::Serial
            }
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_serialmain as u32 => {
                UsbDeviceType::SerialMain
            }
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_serialdisplay as u32 => {
                UsbDeviceType::SerialDisplay
            }
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_massstorage as u32 => {
                UsbDeviceType::MassStorage
            }
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_esp32 as u32 => {
                UsbDeviceType::Esp32
            }
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_ftdi as u32 => UsbDeviceType::Ftdi,
            _ => UsbDeviceType::Other,
        };

        Ok(UsbDevice {
            kind,
            vid,
            pid,
            name,
            serial,
            location,
            port,
            paths,
        })
    }

    /// Helper method to get USB device string during enumeration.
    ///
    /// This method retrieves string information from the currently enumerated
    /// USB device. It is used internally during USB device enumeration.
    ///
    /// # Arguments
    ///
    /// * `string_type` - The type of string to retrieve from the USB device
    ///
    /// # Returns
    ///
    /// - `Ok(String)`: The requested string value
    /// - `Err(FreeWiliError)`: If the string cannot be retrieved
    fn get_usb_device_string(
        &self,
        string_type: ffi::_fw_stringtype_t,
    ) -> Result<String, FreeWiliError> {
        let mut buffer = vec![0u8; 1024];
        let buffer_size = buffer.len() as u32;

        let res = unsafe {
            ffi::fw_usb_device_get_str(
                self.handle,
                string_type as u32,
                buffer.as_mut_ptr() as *mut c_char,
                buffer_size,
            )
        };
        if res != ffi::_fw_error_t::fw_error_success as u32 {
            return Err(res.into());
        }

        let cstr = unsafe { CStr::from_ptr(buffer.as_ptr() as *const c_char) };
        Ok(cstr.to_string_lossy().into_owned())
    }

    /// Helper method to get USB device integer during enumeration.
    ///
    /// This method retrieves integer information from the currently enumerated
    /// USB device. It is used internally during USB device enumeration.
    ///
    /// # Arguments
    ///
    /// * `int_type` - The type of integer to retrieve from the USB device
    ///   (VID, PID, location, etc.)
    ///
    /// # Returns
    ///
    /// - `Ok(u32)`: The requested integer value
    /// - `Err(FreeWiliError)`: If the integer cannot be retrieved
    fn get_usb_device_int(&self, int_type: ffi::_fw_inttype_t) -> Result<u32, FreeWiliError> {
        let mut value: u32 = 0;

        let res = unsafe { ffi::fw_usb_device_get_int(self.handle, int_type as u32, &mut value) };
        if res != ffi::_fw_error_t::fw_error_success as u32 {
            return Err(res.into());
        }

        Ok(value)
    }
}

impl fmt::Display for FreeWiliDevice {
    /// Format the device for display.
    ///
    /// This implementation displays the device in the format "{name} {serial}",
    /// with fallback values if the information cannot be retrieved.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use freewili_finder_rs::FreeWiliDevice;
    ///
    /// let devices = FreeWiliDevice::find_all()?;
    /// for device in devices {
    ///     println!("{}", device); // Displays: "Intrepid FreeWili FW5419"
    /// }
    /// # Ok::<(), freewili_finder_rs::FreeWiliError>(())
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Get device name and serial, with fallbacks
        let name = self
            .get_device_string(fw_stringtype_name)
            .unwrap_or_else(|_| "Unknown Device".to_string());
        let serial = self
            .get_device_string(fw_stringtype_serial)
            .unwrap_or_else(|_| "Unknown Serial".to_string());

        write!(f, "{name} {serial}")
    }
}

/// Tests and examples for the FreeWili Finder library.
///
/// These tests demonstrate the usage patterns and verify the functionality
/// of the library.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test error type conversions from the C library.
    #[test]
    fn test_device_type_conversion() {
        // Test From<ffi::_fw_error_t> for FreeWiliError
        let error = FreeWiliError::from(ffi::_fw_error_t::fw_error_invalid_parameter);
        assert!(matches!(error, FreeWiliError::InvalidParameter));

        // Test From<fw_error_t> for FreeWiliError
        let error = FreeWiliError::from(ffi::_fw_error_t::fw_error_memory as u32);
        assert!(matches!(error, FreeWiliError::MemoryError));
    }

    /// Test USB device type enumeration.
    #[test]
    fn test_usb_device_type_enum() {
        // Test that the enum variants are properly defined
        assert_eq!(UsbDeviceType::Hub, UsbDeviceType::Hub);
        assert_ne!(UsbDeviceType::SerialMain, UsbDeviceType::SerialDisplay);

        // Test that Display implementation works
        use std::fmt::Write;
        let mut output = String::new();

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

        write!(&mut output, "{usb_device}").unwrap();
        assert_eq!(output, "Main: Test Device: /dev/ttyUSB0");
    }

    /// Example of finding devices (will only work if hardware is connected).
    ///
    /// This test serves as a documentation example and will be ignored
    /// unless hardware is present.
    #[test]
    #[ignore = "Requires FreeWili hardware to be connected"]
    fn example_device_discovery() {
        // This is a comprehensive example of how to use the library
        match FreeWiliDevice::find_all() {
            Ok(devices) => {
                println!("Found {} devices", devices.len());

                for (i, device) in devices.iter().enumerate() {
                    println!("Device {}: {}", i + 1, device);

                    if device.is_valid() {
                        // Get device information
                        if let Ok(name) = device.get_device_string(fw_stringtype_name) {
                            println!("  Name: {name}");
                        }

                        if let Ok(serial) = device.get_device_string(fw_stringtype_serial) {
                            println!("  Serial: {serial}");
                        }

                        // Enumerate USB devices
                        if let Ok(usb_devices) = device.get_usb_devices() {
                            println!("  USB Devices: {}", usb_devices.len());
                            for (j, usb_device) in usb_devices.iter().enumerate() {
                                println!("    {}: {}", j + 1, usb_device);
                            }
                        }
                    }
                }
            }
            Err(e) => println!("Failed to find devices: {e}"),
        }
    }
}
