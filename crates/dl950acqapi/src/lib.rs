use std::ffi::{CString, NulError};

use dl950acqapi_sys::{
    ScCloseInstrument, ScExit, ScInit, ScOpenInstrument, SC_ERROR, SC_FREERUN, SC_SUCCESS,
    SC_TRIGGER, SC_TRIGGER_ASYNC, SC_WIRE_HISLIP, SC_WIRE_USBTMC, SC_WIRE_VISAUSB, SC_WIRE_VXI11,
};
use thiserror::Error;

// Every API returns either SC_SUCCESS or SC_ERROR,
// which is super uninformative.
#[derive(Debug)]
pub struct Error {
    error_code: i32,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error_code as u32 {
            SC_ERROR => f.write_str("The API returned SC_ERROR."),
            _ => {
                write!(
                    f,
                    "The API returned an unknown error code ({}).",
                    self.error_code
                )
            }
        }
    }
}
impl std::error::Error for Error {}
impl Error {
    fn report_drop_error(&self) {
        eprintln!("API failure while dropping: {self}")
    }
}

fn check(code: i32) -> Result<(), Error> {
    match code as u32 {
        SC_SUCCESS => Ok(()),
        _ => Err(Error { error_code: code }),
    }
}

// Declare non_exhaustive to prevent construction of the unit struct
#[non_exhaustive]
pub struct Api;

impl Api {
    pub fn init() -> Result<Api, Error> {
        check(unsafe { ScInit() })?;
        Ok(Api)
    }

    pub fn exit(self) -> Result<(), Error> {
        self.exit_impl()
    }

    fn exit_impl(&self) -> Result<(), Error> {
        check(unsafe { ScExit() })
    }
}
impl Drop for Api {
    fn drop(&mut self) {
        if let Err(e) = self.exit_impl() {
            e.report_drop_error();
        }
    }
}

impl Api {
    pub fn open_trigger_asnyc(
        &self,
        wire_type: WireType,
        address: &str,
    ) -> Result<Handle, OpenError> {
        Handle::open(wire_type, address, ConnectionMode::TriggerAsync)
    }
}

pub struct Handle(i32);
pub enum WireType {
    /// USBTMC(YTUSB)
    UsbTmc,
    /// VISAUSB
    VisaUsb,
    /// VXI-11
    Vxi11,
    /// HiSLIP
    HiSlip,
}
enum ConnectionMode {
    Freerun,
    Trigger,
    TriggerAsync,
}

#[derive(Debug, Error)]
pub enum OpenError {
    #[error("The provided address contained null characters: {0}")]
    AddressContainedNul(#[from] NulError),
    #[error("The API returned an error: {0}")]
    ApiError(#[from] Error),
}

impl Handle {
    fn open(wire_type: WireType, address: &str, mode: ConnectionMode) -> Result<Handle, OpenError> {
        let wire = match wire_type {
            WireType::UsbTmc => SC_WIRE_USBTMC,
            WireType::VisaUsb => SC_WIRE_VISAUSB,
            WireType::Vxi11 => SC_WIRE_VXI11,
            WireType::HiSlip => SC_WIRE_HISLIP,
        } as i32;
        let address = CString::new(address)?;
        let address = address.as_ptr() as _;
        let mode = match mode {
            ConnectionMode::Freerun => SC_FREERUN,
            ConnectionMode::Trigger => SC_TRIGGER,
            ConnectionMode::TriggerAsync => SC_TRIGGER_ASYNC,
        } as i32;
        let mut handle = 0;
        let handle_ptr = &mut handle as *mut i32;
        check(unsafe { ScOpenInstrument(wire, address, mode, handle_ptr) })?;
        Ok(Handle(handle))
    }

    pub fn close(self) -> Result<(), Error> {
        self.close_impl()
    }

    fn close_impl(&self) -> Result<(), Error> {
        check(unsafe { ScCloseInstrument(self.0) })
    }
}
impl Drop for Handle {
    fn drop(&mut self) {
        if let Err(e) = self.close_impl() {
            e.report_drop_error();
        }
    }
}
