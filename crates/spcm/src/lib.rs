use std::{
    ffi::{c_ulong, c_void, CString, NulError},
    fmt::{Debug, Display},
    ptr::null_mut,
    string::FromUtf8Error,
};

use bstr::BString;
use spcm_sys::*;
use thiserror::Error;

pub struct Device(*mut c_void);

#[derive(Debug, Error)]
pub enum OpenError {
    #[error("The provided address contains null characters: {0}")]
    AddressContainedNul(#[from] NulError),
    #[error("Failed to open device on {0:?}")]
    OpenError(String),
}

// Open and close
impl Device {
    pub fn open(address: &str) -> Result<Device, OpenError> {
        let handle = {
            let address = CString::new(address)?;
            let address = address.as_ptr() as _;
            unsafe { spcm_hOpen(address) }
        };
        if handle.is_null() {
            Err(OpenError::OpenError(address.to_owned()))
        } else {
            Ok(Device(handle))
        }
    }

    pub fn close(self) {
        self.close_impl()
    }

    pub fn close_impl(&self) {}
}
impl Drop for Device {
    fn drop(&mut self) {
        self.close_impl()
    }
}

// Error handling
#[derive(Debug, Error)]
pub struct Error {
    api_error_code: ErrorCode,
    error_code: ErrorCode,
    message: BString,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "API error code: {:?}, Error code {:?}, message: {:?}",
            self.api_error_code, self.error_code, self.message
        )
    }
}

struct ErrorCode(c_ulong);
impl Debug for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        // Copy pasted from bindgen-generated source
        // TODO: is there a way to auto-generate?
        let name = match self.0 {
            ERR_OK => Some("ERR_OK"),
            ERR_INIT => Some("ERR_INIT"),
            ERR_NR => Some("ERR_NR"),
            ERR_TYP => Some("ERR_TYP"),
            ERR_FNCNOTSUPPORTED => Some("ERR_FNCNOTSUPPORTED"),
            ERR_BRDREMAP => Some("ERR_BRDREMAP"),
            ERR_KERNELVERSION => Some("ERR_KERNELVERSION"),
            ERR_HWDRVVERSION => Some("ERR_HWDRVVERSION"),
            ERR_ADRRANGE => Some("ERR_ADRRANGE"),
            ERR_INVALIDHANDLE => Some("ERR_INVALIDHANDLE"),
            ERR_BOARDNOTFOUND => Some("ERR_BOARDNOTFOUND"),
            ERR_BOARDINUSE => Some("ERR_BOARDINUSE"),
            ERR_EXPHW64BITADR => Some("ERR_EXPHW64BITADR"),
            ERR_FWVERSION => Some("ERR_FWVERSION"),
            ERR_SYNCPROTOCOL => Some("ERR_SYNCPROTOCOL"),
            ERR_KERNEL => Some("ERR_KERNEL"),
            ERR_LASTERR => Some("ERR_LASTERR"),
            ERR_ABORT => Some("ERR_ABORT"),
            ERR_BOARDLOCKED => Some("ERR_BOARDLOCKED"),
            ERR_DEVICE_MAPPING => Some("ERR_DEVICE_MAPPING"),
            ERR_NETWORKSETUP => Some("ERR_NETWORKSETUP"),
            ERR_NETWORKTRANSFER => Some("ERR_NETWORKTRANSFER"),
            ERR_FWPOWERCYCLE => Some("ERR_FWPOWERCYCLE"),
            ERR_NETWORKTIMEOUT => Some("ERR_NETWORKTIMEOUT"),
            ERR_BUFFERSIZE => Some("ERR_BUFFERSIZE"),
            ERR_RESTRICTEDACCESS => Some("ERR_RESTRICTEDACCESS"),
            ERR_INVALIDPARAM => Some("ERR_INVALIDPARAM"),
            ERR_TEMPERATURE => Some("ERR_TEMPERATURE"),
            ERR_FAN => Some("ERR_FAN"),
            ERR_GOLDENIMAGE => Some("ERR_GOLDENIMAGE"),
            ERR_REG => Some("ERR_REG"),
            ERR_VALUE => Some("ERR_VALUE"),
            ERR_FEATURE => Some("ERR_FEATURE"),
            ERR_SEQUENCE => Some("ERR_SEQUENCE"),
            ERR_READABORT => Some("ERR_READABORT"),
            ERR_NOACCESS => Some("ERR_NOACCESS"),
            ERR_POWERDOWN => Some("ERR_POWERDOWN"),
            ERR_TIMEOUT => Some("ERR_TIMEOUT"),
            ERR_CALLTYPE => Some("ERR_CALLTYPE"),
            ERR_EXCEEDSINT32 => Some("ERR_EXCEEDSINT32"),
            ERR_NOWRITEALLOWED => Some("ERR_NOWRITEALLOWED"),
            ERR_SETUP => Some("ERR_SETUP"),
            ERR_CLOCKNOTLOCKED => Some("ERR_CLOCKNOTLOCKED"),
            ERR_MEMINIT => Some("ERR_MEMINIT"),
            ERR_POWERSUPPLY => Some("ERR_POWERSUPPLY"),
            ERR_ADCCOMMUNICATION => Some("ERR_ADCCOMMUNICATION"),
            ERR_CHANNEL => Some("ERR_CHANNEL"),
            ERR_NOTIFYSIZE => Some("ERR_NOTIFYSIZE"),
            ERR_TOOSMALL => Some("ERR_TOOSMALL"),
            ERR_RUNNING => Some("ERR_RUNNING"),
            ERR_ADJUST => Some("ERR_ADJUST"),
            ERR_PRETRIGGERLEN => Some("ERR_PRETRIGGERLEN"),
            ERR_DIRMISMATCH => Some("ERR_DIRMISMATCH"),
            ERR_POSTEXCDSEGMENT => Some("ERR_POSTEXCDSEGMENT"),
            ERR_SEGMENTINMEM => Some("ERR_SEGMENTINMEM"),
            ERR_MULTIPLEPW => Some("ERR_MULTIPLEPW"),
            ERR_NOCHANNELPWOR => Some("ERR_NOCHANNELPWOR"),
            ERR_ANDORMASKOVRLAP => Some("ERR_ANDORMASKOVRLAP"),
            ERR_ANDMASKEDGE => Some("ERR_ANDMASKEDGE"),
            ERR_ORMASKLEVEL => Some("ERR_ORMASKLEVEL"),
            ERR_EDGEPERMOD => Some("ERR_EDGEPERMOD"),
            ERR_DOLEVELMINDIFF => Some("ERR_DOLEVELMINDIFF"),
            ERR_STARHUBENABLE => Some("ERR_STARHUBENABLE"),
            ERR_PATPWSMALLEDGE => Some("ERR_PATPWSMALLEDGE"),
            ERR_XMODESETUP => Some("ERR_XMODESETUP"),
            ERR_AVRG_TDA => Some("ERR_AVRG_TDA"),
            ERR_NOPCI => Some("ERR_NOPCI"),
            ERR_PCIVERSION => Some("ERR_PCIVERSION"),
            ERR_PCINOBOARDS => Some("ERR_PCINOBOARDS"),
            ERR_PCICHECKSUM => Some("ERR_PCICHECKSUM"),
            ERR_DMALOCKED => Some("ERR_DMALOCKED"),
            ERR_MEMALLOC => Some("ERR_MEMALLOC"),
            ERR_EEPROMLOAD => Some("ERR_EEPROMLOAD"),
            ERR_CARDNOSUPPORT => Some("ERR_CARDNOSUPPORT"),
            ERR_CONFIGACCESS => Some("ERR_CONFIGACCESS"),
            ERR_FIFOBUFOVERRUN => Some("ERR_FIFOBUFOVERRUN"),
            ERR_FIFOHWOVERRUN => Some("ERR_FIFOHWOVERRUN"),
            ERR_FIFOFINISHED => Some("ERR_FIFOFINISHED"),
            ERR_FIFOSETUP => Some("ERR_FIFOSETUP"),
            ERR_TIMESTAMP_SYNC => Some("ERR_TIMESTAMP_SYNC"),
            ERR_STARHUB => Some("ERR_STARHUB"),
            ERR_INTERNAL_ERROR => Some("ERR_INTERNAL_ERROR"),
            _ => None,
        };
        if let Some(name) = name {
            write!(f, " /* {} */", name)?;
        }
        Ok(())
    }
}

impl Device {
    fn check(&self, api_error_code: c_ulong) -> Result<(), Error> {
        let mut error_buffer = vec![0; ERRORTEXTLEN as usize];
        let error_buffer_ptr = error_buffer.as_mut_ptr() as _;
        let error_code =
            unsafe { spcm_dwGetErrorInfo_i32(self.0, null_mut(), null_mut(), error_buffer_ptr) };
        if api_error_code != ERR_OK || error_code != ERR_OK {
            if let Some(pos) = error_buffer.iter().position(|&x| x == 0) {
                error_buffer.truncate(pos);
            }
            Err(Error {
                api_error_code: ErrorCode(api_error_code),
                error_code: ErrorCode(error_code),
                message: BString::new(error_buffer),
            })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Error)]
pub enum DeviceCardTypeError {
    #[error("API returned an error: {0:?}")]
    ApiError(#[from] Error),
    #[error("Card type does not contain a null byte: {0:?}")]
    NullCharacterNotPresent(BString),
    #[error("Card type cannot be converted to a UTF-8 string: {0:?}")]
    InvalidString(#[from] FromUtf8Error),
}

impl Device {
    pub fn card_type_str(&self) -> Result<String, DeviceCardTypeError> {
        // We assume that 1024 bytes is sufficiently long for card type.
        let mut buffer = vec![0; 1024];
        let buffer_ptr = buffer.as_mut_ptr() as *mut c_void;
        let buffer_len = buffer.len() as _;
        self.check(unsafe {
            spcm_dwGetParam_ptr(self.0, SPC_PCITYP as _, buffer_ptr, buffer_len)
        })?;

        let Some(len) = buffer.iter().position(|&x| x == 0) else {
            return Err(DeviceCardTypeError::NullCharacterNotPresent(BString::new(
                buffer,
            )));
        };
        buffer.truncate(len);

        Ok(String::from_utf8(buffer)?)
    }
}
