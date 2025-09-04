//! # FreeWili Finder - Rust Bindings
//!
//! This library provides safe Rust bindings for the FreeWili Finder C/C++ library,
//! making it easy to discover and interface with FreeWili devices from Rust applications.
//!
mod ffi;

use ffi::fw_error_t;
use ffi::fw_freewili_device_t;
use std::ffi::{CStr, c_char};
use std::fmt;
use std::ptr;
use thiserror::Error;

use ffi::_fw_inttype_t::*;
use ffi::_fw_stringtype_t::*;
use ffi::_fw_devicetype_t::*;

use crate::ffi::fw_stringtype_t;

#[derive(Error, Debug)]
pub enum FreeWiliError {
    /// Invalid parameter passed to a function
    #[error("Invalid Parameter")]
    InvalidParameter,
    /// Invalid device handle (device may have been disconnected)
    #[error("Invalid device handle")]
    InvalidDevice,
    /// Internal error in the underlying C library
    #[error("Internal error{}", .0.as_ref().map(|s| format!(": {s}")).unwrap_or_default())]
    InternalError(Option<String>),
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

pub type Result<T> = std::result::Result<T, FreeWiliError>;

impl From<ffi::_fw_error_t> for FreeWiliError {
    fn from(error: ffi::_fw_error_t) -> Self {
        match error {
            ffi::_fw_error_t::fw_error_success => FreeWiliError::None,
            ffi::_fw_error_t::fw_error_invalid_parameter => FreeWiliError::InvalidParameter,
            ffi::_fw_error_t::fw_error_invalid_device => FreeWiliError::InvalidDevice,
            ffi::_fw_error_t::fw_error_internal_error => FreeWiliError::InternalError(None),
            ffi::_fw_error_t::fw_error_memory => FreeWiliError::MemoryError,
            ffi::_fw_error_t::fw_error_no_more_devices => FreeWiliError::NoMoreDevices,
            ffi::_fw_error_t::fw_error_none => FreeWiliError::None,
            ffi::_fw_error_t::fw_error__maxvalue => FreeWiliError::InternalError(None),
        }
    }
}

impl From<fw_error_t> for FreeWiliError {
    fn from(error_code: fw_error_t) -> Self {
        match error_code {
            x if x == ffi::_fw_error_t::fw_error_success as u32 => FreeWiliError::None,
            x if x == ffi::_fw_error_t::fw_error_invalid_parameter as u32 => FreeWiliError::InvalidParameter,
            x if x == ffi::_fw_error_t::fw_error_invalid_device as u32 => FreeWiliError::InvalidDevice,
            x if x == ffi::_fw_error_t::fw_error_internal_error as u32 => FreeWiliError::InternalError(None),
            x if x == ffi::_fw_error_t::fw_error_memory as u32 => FreeWiliError::MemoryError,
            x if x == ffi::_fw_error_t::fw_error_no_more_devices as u32 => FreeWiliError::NoMoreDevices,
            x if x == ffi::_fw_error_t::fw_error_none as u32 => FreeWiliError::None,
            x if x == ffi::_fw_error_t::fw_error__maxvalue as u32 => FreeWiliError::InternalError(None),
            _ => FreeWiliError::InternalError(Some(format!("Unknown error code: {}", error_code))),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbDeviceType {
    /// USB Hub
    Hub,
    /// Legacy Serial device
    Serial,
    /// Main CPU Serial
    SerialMain,
    /// Display CPU Serial
    SerialDisplay,
    /// Mass Storage Device
    MassStorage,
    /// ESP32 device
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

impl From<ffi::fw_usbdevicetype_t> for UsbDeviceType {
    fn from(device_type: ffi::fw_usbdevicetype_t) -> Self {
        match device_type {
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_hub as u32 => UsbDeviceType::Hub,
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_serial as u32 => UsbDeviceType::Serial,
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_serialmain as u32 => UsbDeviceType::SerialMain,
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_serialdisplay as u32 => UsbDeviceType::SerialDisplay,
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_massstorage as u32 => UsbDeviceType::MassStorage,
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_esp32 as u32 => UsbDeviceType::Esp32,
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_ftdi as u32 => UsbDeviceType::Ftdi,
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype_other as u32 => UsbDeviceType::Other,
            x if x == ffi::_fw_usbdevicetype_t::fw_usbdevicetype__maxvalue as u32 => UsbDeviceType::_MaxValue,
            _ => UsbDeviceType::Other, // Default fallback
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    /// USB Hub (parent device)
    Unknown,
    /// Generic Serial Port device
    Freewili,
    /// Main CPU Serial Port
    Defcon2024Badge,
    /// Display CPU Serial Port
    Defcon2025FwBadge,
    /// Mass Storage Device
    Uf2,
    /// ESP32 USB device (JTAG/RTT)
    Winky,
}

impl From<ffi::_fw_devicetype_t> for DeviceType {
    fn from(device_type: ffi::_fw_devicetype_t) -> Self {
        match device_type {
            ffi::_fw_devicetype_t::fw_devicetype_unknown => DeviceType::Unknown,
            ffi::_fw_devicetype_t::fw_devicetype_freewili => DeviceType::Freewili,
            ffi::_fw_devicetype_t::fw_devicetype_defcon2024badge => DeviceType::Defcon2024Badge,
            ffi::_fw_devicetype_t::fw_devicetype_defcon2025fwbadge => DeviceType::Defcon2025FwBadge,
            ffi::_fw_devicetype_t::fw_devicetype_uf2 => DeviceType::Uf2,
            ffi::_fw_devicetype_t::fw_devicetype_winky => DeviceType::Winky,
        }
    }
}

impl From<ffi::fw_devicetype_t> for DeviceType {
    fn from(device_type: ffi::fw_devicetype_t) -> Self {
        match device_type {
            x if x == ffi::_fw_devicetype_t::fw_devicetype_unknown as u32 => DeviceType::Unknown,
            x if x == ffi::_fw_devicetype_t::fw_devicetype_freewili as u32 => DeviceType::Freewili,
            x if x == ffi::_fw_devicetype_t::fw_devicetype_defcon2024badge as u32 => DeviceType::Defcon2024Badge,
            x if x == ffi::_fw_devicetype_t::fw_devicetype_defcon2025fwbadge as u32 => DeviceType::Defcon2025FwBadge,
            x if x == ffi::_fw_devicetype_t::fw_devicetype_uf2 as u32 => DeviceType::Uf2,
            x if x == ffi::_fw_devicetype_t::fw_devicetype_winky as u32 => DeviceType::Winky,
            _ => DeviceType::Unknown, // Default fallback
        }
    }
}

#[derive(Debug, Clone)]
pub struct USBDevice {
    /// The type of USB device
    pub kind: UsbDeviceType,
    pub kind_name: String,

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
    /// USB Port chain
    pub port_chain: Vec<u32>,
    /// Serial port path (for serial devices like /dev/ttyUSB0, COM1)
    pub port: Option<String>,
    /// File system path
    pub path: Option<String>,
}

impl USBDevice {
    /// # Safety
    /// 
    /// The `device` pointer must be a valid pointer to a `fw_freewili_device_t` that is properly initialized
    /// and has not been freed. The caller must ensure the device remains valid for the duration of this call.
    pub unsafe fn from_device(device: *mut ffi::fw_freewili_device_t) -> Result<Self> {
        let mut usb_device_type: ffi::fw_usbdevicetype_t = fw_devicetype_unknown as ffi::fw_usbdevicetype_t;
        let res = unsafe { ffi::fw_usb_device_get_type(device, &mut usb_device_type) };
        if res != ffi::_fw_error_t::fw_error_success as ffi::fw_error_t {
            return Err(res.into());
        }

        let mut usb_device_type_name = vec![0i8; 1024];
        let mut usb_device_type_name_size: u32 = usb_device_type_name.len() as u32;
        let res = unsafe {
            ffi::fw_usb_device_get_type_name(
                usb_device_type as ffi::fw_usbdevicetype_t,
                usb_device_type_name.as_mut_ptr(),
                &mut usb_device_type_name_size,
            )
        };
        if res != ffi::_fw_error_t::fw_error_success as ffi::fw_error_t {
            return Err(res.into());
        }
        let usb_device_type_name =
            unsafe { CStr::from_ptr(usb_device_type_name.as_ptr() as *const c_char) }
                .to_string_lossy()
                .into_owned();

        let mut vid: u32 = 0;
        let res = unsafe {
            ffi::fw_usb_device_get_int(device, fw_inttype_vid as u32, &mut vid as *mut u32)
        };
        if res != ffi::_fw_error_t::fw_error_success as ffi::fw_error_t {
            return Err(res.into());
        }

        let mut pid: u32 = 0;
        let res = unsafe {
            ffi::fw_usb_device_get_int(device, fw_inttype_pid as u32, &mut pid as *mut u32)
        };
        if res != ffi::_fw_error_t::fw_error_success as ffi::fw_error_t {
            return Err(res.into());
        }

        let mut location: u32 = 0;
        let res = unsafe {
            ffi::fw_usb_device_get_int(
                device,
                fw_inttype_location as u32,
                &mut location as *mut u32,
            )
        };
        if res != ffi::_fw_error_t::fw_error_success as ffi::fw_error_t {
            return Err(res.into());
        }

        let mut name = vec![0i8; 1024];
        let mut name_size: u32 = name.len() as u32;
        let res = unsafe {
            ffi::fw_usb_device_get_str(
                device,
                fw_stringtype_name as fw_stringtype_t,
                name.as_mut_ptr(),
                &mut name_size,
            )
        };
        if res != ffi::_fw_error_t::fw_error_success as ffi::fw_error_t {
            return Err(res.into());
        }
        let name = unsafe { CStr::from_ptr(name.as_ptr() as *const c_char) }
            .to_string_lossy()
            .into_owned();

        let mut serial = vec![0i8; 1024];
        let mut serial_size: u32 = serial.len() as u32;
        let res = unsafe {
            ffi::fw_usb_device_get_str(
                device,
                fw_stringtype_serial as fw_stringtype_t,
                serial.as_mut_ptr(),
                &mut serial_size,
            )
        };
        if res != ffi::_fw_error_t::fw_error_success as ffi::fw_error_t {
            return Err(res.into());
        }
        let serial = unsafe { CStr::from_ptr(serial.as_ptr() as *const c_char) }
            .to_string_lossy()
            .into_owned();

        let mut port = vec![0i8; 1024];
        let mut port_size: u32 = port.len() as u32;
        let res = unsafe {
            ffi::fw_usb_device_get_str(
                device,
                fw_stringtype_port as fw_stringtype_t,
                port.as_mut_ptr(),
                &mut port_size,
            )
        };
        if res != ffi::_fw_error_t::fw_error_success as ffi::fw_error_t
            && res != ffi::_fw_error_t::fw_error_none as ffi::fw_error_t
        {
            return Err(res.into());
        }
        let port = unsafe { CStr::from_ptr(port.as_ptr() as *const c_char) }
            .to_string_lossy()
            .into_owned();

        let mut path = vec![0i8; 1024];
        let mut path_size: u32 = path.len() as u32;
        let res = unsafe {
            ffi::fw_usb_device_get_str(
                device,
                fw_stringtype_path as fw_stringtype_t,
                path.as_mut_ptr(),
                &mut path_size,
            )
        };
        if res != ffi::_fw_error_t::fw_error_success as ffi::fw_error_t
            && res != ffi::_fw_error_t::fw_error_none as ffi::fw_error_t
        {
            return Err(res.into());
        }
        let path = unsafe { CStr::from_ptr(path.as_ptr() as *const c_char) }
            .to_string_lossy()
            .into_owned();

        let mut port_chain: Vec<u32> = vec![0u32; 10];
        let mut port_chain_size: u32 = port_chain.len() as u32;
        let res = unsafe {
            ffi::fw_usb_device_get_port_chain(device, port_chain.as_mut_ptr(), &mut port_chain_size)
        };
        if res != ffi::_fw_error_t::fw_error_success as ffi::fw_error_t {
            return Err(res.into());
        }
        port_chain.resize(port_chain_size as usize, 0);

        let usb_device = USBDevice {
            kind: usb_device_type.into(),
            kind_name: usb_device_type_name,
            vid: vid as u16,
            pid: pid as u16,
            name,
            serial,
            location,
            port_chain,
            port: if port.is_empty() { None } else { Some(port) },
            path: if path.is_empty() { None } else { Some(path) },
        };

        Ok(usb_device)
    }
}

impl fmt::Display for USBDevice {
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
        } else if let Some(path) = &self.path {
            write!(f, ": {path}")?;
        }

        Ok(())
    }
}

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
    pub fn find_all() -> Result<Vec<FreeWiliDevice>> {
        const MAX_DEVICE_COUNT: u32 = 255;
        let mut device_count: u32 = MAX_DEVICE_COUNT;
        let mut devices: [*mut fw_freewili_device_t; MAX_DEVICE_COUNT as usize] =
            [std::ptr::null_mut(); MAX_DEVICE_COUNT as usize];
        let mut error_msg = vec![0u8; 1024];
        let mut error_size: u32 = error_msg.len() as u32;

        let res: fw_error_t = unsafe {
            ffi::fw_device_find_all(
                devices.as_mut_ptr(),
                &mut device_count,
                error_msg.as_mut_ptr() as *mut i8,
                &mut error_size,
            )
        };
        match res {
            x if x == ffi::_fw_error_t::fw_error_success as ffi::fw_error_t => {}
            x if x == ffi::_fw_error_t::fw_error_internal_error as ffi::fw_error_t => {
                let error_str = unsafe { CStr::from_ptr(error_msg.as_ptr() as *const c_char) }
                    .to_string_lossy()
                    .into_owned();
                return Err(FreeWiliError::InternalError(Some(error_str)));
            }
            _ => return Err(res.into()),
        }

        let mut device_handles = Vec::with_capacity(device_count as usize);
        for i in 0..device_count {
            device_handles.push(FreeWiliDevice {
                handle: devices[i as usize],
            });
        }
        Ok(device_handles)
    }

    pub fn device_type(&self) -> Result<DeviceType> {
        let mut device_type: ffi::fw_devicetype_t = 0;
        let res = unsafe { ffi::fw_device_get_type(self.handle, &mut device_type) };
        if res != ffi::_fw_error_t::fw_error_success as u32 {
            return Err(res.into());
        }

        Ok(device_type.into())
    }

    pub fn device_type_name(&self) -> Result<String> {
        let device_type = self.device_type()? as ffi::fw_devicetype_t;

        let mut device_type_name = vec![0i8; 1024];
        let mut device_type_name_size: u32 = device_type_name.len() as u32;
        let res = unsafe {
            ffi::fw_device_get_type_name(
                device_type,
                device_type_name.as_mut_ptr(),
                &mut device_type_name_size,
            )
        };
        if res != ffi::_fw_error_t::fw_error_success as ffi::fw_error_t {
            return Err(res.into());
        }

        let cstr = unsafe { CStr::from_ptr(device_type_name.as_ptr() as *const c_char) };
        Ok(cstr.to_string_lossy().into_owned())
    }

    fn get_device_string(
        &self,
        string_type: ffi::_fw_stringtype_t,
    ) -> Result<String> {
        let mut buffer = vec![0u8; 1024];
        let mut buffer_size = buffer.len() as u32;

        let res: fw_error_t = unsafe {
            ffi::fw_device_get_str(
                self.handle,
                string_type as u32,
                buffer.as_mut_ptr() as *mut c_char,
                &mut buffer_size,
            )
        };
        if res != ffi::_fw_error_t::fw_error_success as u32 {
            return Err(res.into());
        }

        let cstr = unsafe { CStr::from_ptr(buffer.as_ptr() as *const c_char) };
        Ok(cstr.to_string_lossy().into_owned())
    }

    pub fn name(&self) -> Result<String> {
        self.get_device_string(ffi::_fw_stringtype_t::fw_stringtype_name)
    }

    pub fn serial(&self) -> Result<String> {
        self.get_device_string(ffi::_fw_stringtype_t::fw_stringtype_serial)
    }

    pub fn unique_id(&self) -> Result<u64> {
        let mut unique_id: u64 = 0;
        let res = unsafe { ffi::fw_device_unique_id(self.handle, &mut unique_id as *mut u64) };
        if res != ffi::_fw_error_t::fw_error_success as u32 {
            return Err(res.into());
        }
        Ok(unique_id)
    }

    pub fn standalone(&self) -> Result<bool> {
        let mut is_standalone: bool = false;
        let res =
            unsafe { ffi::fw_device_is_standalone(self.handle, &mut is_standalone as *mut bool) };
        if res != ffi::_fw_error_t::fw_error_success as u32 {
            return Err(res.into());
        }
        Ok(is_standalone)
    }

    pub fn usb_device_get_string(
        &self,
        string_type: ffi::_fw_stringtype_t,
    ) -> Result<String> {
        let mut buffer = vec![0u8; 1024];
        let mut buffer_size = buffer.len() as u32;

        let res: fw_error_t = unsafe {
            ffi::fw_usb_device_get_str(
                self.handle,
                string_type as u32,
                buffer.as_mut_ptr() as *mut c_char,
                &mut buffer_size,
            )
        };
        if res != ffi::_fw_error_t::fw_error_success as u32 {
            return Err(res.into());
        }

        let cstr = unsafe { CStr::from_ptr(buffer.as_ptr() as *const c_char) };
        Ok(cstr.to_string_lossy().into_owned())
    }

    pub fn get_usb_devices(&self) -> Result<Vec<USBDevice>> {
        let res = unsafe { ffi::fw_usb_device_begin(self.handle) };
        if res != ffi::_fw_error_t::fw_error_success as u32 {
            return Err(res.into());
        }

        let mut devices = Vec::new();
        loop {
            let usb_device = unsafe { USBDevice::from_device(self.handle)? };
            devices.push(usb_device);

            let res = unsafe { ffi::fw_usb_device_next(self.handle) };
            if res != ffi::_fw_error_t::fw_error_success as u32 {
                break;
            }
        }
        Ok(devices)
    }

    pub fn get_main_usb_device(&self) -> Result<USBDevice> {
        let mut error_msg = vec![0u8; 1024];
        let mut error_size: u32 = error_msg.len() as u32;
        let res = unsafe {
            ffi::fw_usb_device_set(
                self.handle,
                ffi::_fw_usbdevice_iter_set_t::fw_usbdevice_iter_main as ffi::fw_usbdevice_iter_set_t,
                error_msg.as_mut_ptr() as *mut i8,
                &mut error_size,
            )
        };
        if res == ffi::_fw_error_t::fw_error_internal_error as fw_error_t {
            let error_str = unsafe { CStr::from_ptr(error_msg.as_ptr() as *const c_char) }
                    .to_string_lossy()
                    .into_owned();
            return Err(FreeWiliError::InternalError(Some(error_str)));
        }
        if res != ffi::_fw_error_t::fw_error_success as fw_error_t {
            return Err(res.into());
        }
        unsafe { USBDevice::from_device(self.handle) }
    }

    pub fn get_display_usb_device(&self) -> Result<USBDevice> {
        let mut error_msg = vec![0u8; 1024];
        let mut error_size: u32 = error_msg.len() as u32;
        let res = unsafe {
            ffi::fw_usb_device_set(
                self.handle,
                ffi::_fw_usbdevice_iter_set_t::fw_usbdevice_iter_display as ffi::fw_usbdevice_iter_set_t,
                error_msg.as_mut_ptr() as *mut i8,
                &mut error_size,
            )
        };
        if res == ffi::_fw_error_t::fw_error_internal_error as fw_error_t {
            let error_str = unsafe { CStr::from_ptr(error_msg.as_ptr() as *const c_char) }
                    .to_string_lossy()
                    .into_owned();
            return Err(FreeWiliError::InternalError(Some(error_str)));
        }
        if res != ffi::_fw_error_t::fw_error_success as fw_error_t {
            return Err(res.into());
        }
        unsafe { USBDevice::from_device(self.handle) }
    }

    pub fn get_fpga_usb_device(&self) -> Result<USBDevice> {
        let mut error_msg = vec![0u8; 1024];
        let mut error_size: u32 = error_msg.len() as u32;
        let res = unsafe {
            ffi::fw_usb_device_set(
                self.handle,
                ffi::_fw_usbdevice_iter_set_t::fw_usbdevice_iter_fpga as ffi::fw_usbdevice_iter_set_t,
                error_msg.as_mut_ptr() as *mut i8,
                &mut error_size,
            )
        };
        if res == ffi::_fw_error_t::fw_error_internal_error as fw_error_t {
            let error_str = unsafe { CStr::from_ptr(error_msg.as_ptr() as *const c_char) }
                    .to_string_lossy()
                    .into_owned();
            return Err(FreeWiliError::InternalError(Some(error_str)));
        }
        if res != ffi::_fw_error_t::fw_error_success as fw_error_t {
            return Err(res.into());
        }
        unsafe { USBDevice::from_device(self.handle) }
    }

    pub fn get_hub_usb_device(&self) -> Result<USBDevice> {
        let mut error_msg = vec![0u8; 1024];
        let mut error_size: u32 = error_msg.len() as u32;
        let res = unsafe {
            ffi::fw_usb_device_set(
                self.handle,
                ffi::_fw_usbdevice_iter_set_t::fw_usbdevice_iter_hub as ffi::fw_usbdevice_iter_set_t,
                error_msg.as_mut_ptr() as *mut i8,
                &mut error_size,
            )
        };
        if res == ffi::_fw_error_t::fw_error_internal_error as fw_error_t {
            let error_str = unsafe { CStr::from_ptr(error_msg.as_ptr() as *const c_char) }
                    .to_string_lossy()
                    .into_owned();
            return Err(FreeWiliError::InternalError(Some(error_str)));
        }
        if res != ffi::_fw_error_t::fw_error_success as fw_error_t {
            return Err(res.into());
        }
        unsafe { USBDevice::from_device(self.handle) }
    }
}

impl fmt::Display for FreeWiliDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name().unwrap_or_default(), self.serial().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() -> Result<()> {
        let devices = FreeWiliDevice::find_all()?;

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
}