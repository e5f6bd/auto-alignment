use std::{
    ffi::{c_ulong, c_void, CString, NulError},
    fmt::{Debug, Display},
    num::TryFromIntError,
    ptr::null_mut,
    string::FromUtf8Error,
};

use bitflags::bitflags;
use bstr::BString;
use enumset::{EnumSet, EnumSetType};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
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
pub enum DeviceCardTypeStrError {
    #[error("API returned an error: {0:?}")]
    ApiError(#[from] Error),
    #[error("Card type does not contain a null byte: {0:?}")]
    NullCharacterNotPresent(BString),
    #[error("Card type cannot be converted to a UTF-8 string: {0:?}")]
    InvalidString(#[from] FromUtf8Error),
}
impl Device {
    pub fn card_type_str(&self) -> Result<String, DeviceCardTypeStrError> {
        // We assume that 1024 bytes is sufficiently long for card type.
        let mut buffer = vec![0; 1024];
        let buffer_ptr = buffer.as_mut_ptr() as *mut c_void;
        let buffer_len = buffer.len() as _;
        self.check(unsafe {
            spcm_dwGetParam_ptr(self.0, SPC_PCITYP as _, buffer_ptr, buffer_len)
        })?;

        let Some(len) = buffer.iter().position(|&x| x == 0) else {
            return Err(DeviceCardTypeStrError::NullCharacterNotPresent(
                BString::new(buffer),
            ));
        };
        buffer.truncate(len);

        Ok(String::from_utf8(buffer)?)
    }
}

pub mod card_type;
#[derive(Debug, Error)]
pub enum DeviceCardTypeError {
    #[error("API returned an error: {0:?}")]
    ApiError(#[from] Error),
    #[error("Unknown card type: {0:?}")]
    UnknownType(#[from] TryFromPrimitiveError<card_type::CardType>),
}
impl Device {
    pub fn card_type(&self) -> Result<card_type::CardType, DeviceCardTypeError> {
        let ret = {
            let mut ret = 0;
            let ret_ptr = (&mut ret) as _;
            self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_PCITYP as _, ret_ptr) })?;
            ret
        };
        Ok(ret.try_into()?)
    }
}

#[derive(Debug)]
pub struct SerialNumber(pub i32);
impl Device {
    pub fn serial_no(&self) -> Result<SerialNumber, Error> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_PCISERIALNO as _, ret_ptr) })?;
        Ok(SerialNumber(ret))
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum CardFunctionType {
    AnalogInput = SPCM_TYPE_AI,
    AnalogOutput = SPCM_TYPE_AO,
    DigitalInput = SPCM_TYPE_DI,
    DigitalOutput = SPCM_TYPE_DO,
    DigitalInputOutput = SPCM_TYPE_DIO,
}
#[derive(Debug, Error)]
pub enum CardFunctionTypeError {
    #[error("API returned an error: {0:?}")]
    ApiError(#[from] Error),
    #[error("Unknown card type: {0:?}")]
    OutOfRange(#[from] TryFromIntError),
    #[error("Unknown card type: {0:?}")]
    UnknownCardType(#[from] TryFromPrimitiveError<CardFunctionType>),
}
impl Device {
    pub fn function_type(&self) -> Result<CardFunctionType, CardFunctionTypeError> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_FNCTYPE as _, ret_ptr) })?;
        Ok(u32::try_from(ret)?.try_into()?)
    }
}

#[derive(Debug, TryFromPrimitive, EnumSetType)]
#[repr(u32)]
pub enum ExtendedFeature {
    BlockStatistics,
    BlockAverage,
    BoxcarAverage,
    PulseGenerator,
    DDS,
    Evaluation,
}
#[derive(Debug, Error)]
pub enum CardExtendeedFeatureError {
    #[error("API returned an error: {0:?}")]
    ApiError(#[from] Error),
    #[error("Unknown bit found: {0} ({0:#032b})")]
    UnknownBitFound(u32),
}
impl Device {
    pub fn extended_features(&self) -> Result<EnumSet<ExtendedFeature>, CardExtendeedFeatureError> {
        let mut ret = 0i32;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_PCIEXTFEATURES as _, ret_ptr) })?;
        let ret = ret as u32;
        (0..32)
            .map(|i| 1 << i)
            .filter(|x| x & ret > 0)
            .map(|x| {
                Ok(match x {
                    SPCM_FEAT_EXTFW_SEGSTAT => ExtendedFeature::BlockStatistics,
                    SPCM_FEAT_EXTFW_SEGAVERAGE => ExtendedFeature::BlockAverage,
                    SPCM_FEAT_EXTFW_BOXCAR => ExtendedFeature::BoxcarAverage,
                    SPCM_FEAT_EXTFW_PULSEGEN => ExtendedFeature::PulseGenerator,
                    SPCM_FEAT_EXTFW_DDS => ExtendedFeature::DDS,
                    SPCM_FEAT_EXTFW_EVALUATION => ExtendedFeature::Evaluation,
                    _ => return Err(CardExtendeedFeatureError::UnknownBitFound(ret)),
                })
            })
            .try_fold(EnumSet::new(), |x, y| Ok(x | y?))
    }
}

impl Device {
    pub fn num_modules(&self) -> Result<i32, Error> {
        let mut ret = 0i32;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_MIINST_MODULES as _, ret_ptr) })?;
        Ok(ret)
    }

    pub fn num_channels_per_module(&self) -> Result<i32, Error> {
        let mut ret = 0i32;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_MIINST_CHPERMODULE as _, ret_ptr) })?;
        Ok(ret)
    }
}

impl Device {
    pub fn enabled_channels(&self) -> Result<i32, Error> {
        let mut ret = 0i32;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_CHENABLE as _, ret_ptr) })?;
        Ok(ret)
    }

    pub fn enable_channels(&mut self, channels: i32) -> Result<(), Error> {
        self.check(unsafe { spcm_dwSetParam_i32(self.0, SPC_CHENABLE as _, channels) })
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum CardMode {
    StdSingle = SPC_REP_STD_SINGLE,
    StdMulti = SPC_REP_STD_MULTI,
    StdGate = SPC_REP_STD_GATE,
    FifoSingle = SPC_REP_FIFO_SINGLE,
    FifoMulti = SPC_REP_FIFO_MULTI,
    FifoGate = SPC_REP_FIFO_GATE,
    StdContinuous = SPC_REP_STD_CONTINUOUS,
    StdSinglestart = SPC_REP_STD_SINGLERESTART,
    StdSequence = SPC_REP_STD_SEQUENCE,
    StdDds = SPC_REP_STD_DDS,
}
#[derive(Debug, Error)]
pub enum CardModeError {
    #[error("API returned an error: {0:?}")]
    ApiError(#[from] Error),
    #[error("Unknown card mode: {0:?}")]
    OutOfRange(#[from] TryFromIntError),
    #[error("Unknown card mode: {0:?}")]
    UnknownCardType(#[from] TryFromPrimitiveError<CardMode>),
}
impl Device {
    pub fn card_mode(&self) -> Result<CardMode, CardModeError> {
        let mut ret = 0i32;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_CARDMODE as _, ret_ptr) })?;
        Ok(u32::try_from(ret)?.try_into()?)
    }

    pub fn set_card_mode(&mut self, mode: CardMode) -> Result<(), Error> {
        self.check(unsafe { spcm_dwSetParam_i32(self.0, SPC_CARDMODE as _, mode as u32 as i32) })
    }
}

bitflags! {
    pub struct TriggerMask: i32 {
        const Software  = SPC_TMASK_SOFTWARE as i32;
        const Ext0      = SPC_TMASK_EXT0 as i32;
        const Ext1      = SPC_TMASK_EXT1 as i32;
        const Ext2      = SPC_TMASK_EXT2 as i32;
        const Ext3      = SPC_TMASK_EXT3 as i32;
        const Ext4      = SPC_TMASK_EXT4 as i32;
        const Xio0      = SPC_TMASK_XIO0 as i32;
        const Xio1      = SPC_TMASK_XIO1 as i32;
        const Xio2      = SPC_TMASK_XIO2 as i32;
        const Xio3      = SPC_TMASK_XIO3 as i32;
        const Xio4      = SPC_TMASK_XIO4 as i32;
        const Xio5      = SPC_TMASK_XIO5 as i32;
        const Xio6      = SPC_TMASK_XIO6 as i32;
        const Xio7      = SPC_TMASK_XIO7 as i32;
        const Pxi0      = SPC_TMASK_PXI0 as i32;
        const Pxi1      = SPC_TMASK_PXI1 as i32;
        const Pxi2      = SPC_TMASK_PXI2 as i32;
        const Pxi3      = SPC_TMASK_PXI3 as i32;
        const Pxi4      = SPC_TMASK_PXI4 as i32;
        const Pxi5      = SPC_TMASK_PXI5 as i32;
        const Pxi6      = SPC_TMASK_PXI6 as i32;
        const Pxi7      = SPC_TMASK_PXI7 as i32;
        const PxiStar   = SPC_TMASK_PXISTAR as i32;
        const PxiDStarB = SPC_TMASK_PXIDSTARB as i32;
    }
}
#[derive(Debug, Error)]
pub enum GetTriggerMaskError {
    #[error("API returned an error: {0:?}")]
    ApiError(#[from] Error),
    #[error("Unknown trigger mask: {0} ({0:#032b})")]
    UnknownMask(i32),
}
impl Device {
    pub fn available_trigger_or_mask(&self) -> Result<TriggerMask, GetTriggerMaskError> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_TRIG_AVAILORMASK as _, ret_ptr) })?;
        TriggerMask::from_bits(ret).ok_or(GetTriggerMaskError::UnknownMask(ret))
    }

    pub fn trigger_or_mask(&self) -> Result<TriggerMask, GetTriggerMaskError> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_TRIG_ORMASK as _, ret_ptr) })?;
        TriggerMask::from_bits(ret).ok_or(GetTriggerMaskError::UnknownMask(ret))
    }

    pub fn set_trigger_or_mask(&mut self, mask: TriggerMask) -> Result<(), Error> {
        self.check(unsafe { spcm_dwSetParam_i32(self.0, SPC_TRIG_ORMASK as _, mask.bits()) })
    }

    pub fn available_trigger_and_mask(&self) -> Result<TriggerMask, GetTriggerMaskError> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_TRIG_AVAILANDMASK as _, ret_ptr) })?;
        TriggerMask::from_bits(ret).ok_or(GetTriggerMaskError::UnknownMask(ret))
    }

    pub fn trigger_and_mask(&self) -> Result<TriggerMask, GetTriggerMaskError> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_TRIG_ANDMASK as _, ret_ptr) })?;
        TriggerMask::from_bits(ret).ok_or(GetTriggerMaskError::UnknownMask(ret))
    }

    pub fn set_trigger_and_mask(&mut self, mask: TriggerMask) -> Result<(), Error> {
        self.check(unsafe { spcm_dwSetParam_i32(self.0, SPC_TRIG_ANDMASK as _, mask.bits()) })
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(i32)]
pub enum ClockMode {
    InternalPll = SPC_CM_INTPLL as _,
    Quartz1 = SPC_CM_QUARTZ1 as _,
    Quartz2 = SPC_CM_QUARTZ2 as _,
    External = SPC_CM_EXTERNAL as _,
    // External0 = SPC_CM_EXTERNAL0 as _,
    ExternalDivider = SPC_CM_EXTDIVIDER as _,
    ExternalReferenceClock = SPC_CM_EXTREFCLOCK as _,
    PxiReferenceClock = SPC_CM_PXIREFCLOCK as _,
    StarHubDirect = SPC_CM_SHDIRECT as _,
    Quartz2DirectSync = SPC_CM_QUARTZ2_DIRSYNC as _,
    Quartz1DirectSync = SPC_CM_QUARTZ1_DIRSYNC as _,
    External1 = SPC_CM_EXTERNAL1 as _,
    SyncIntenal = SPC_CM_SYNCINT as _,
    SyncExternal = SPC_CM_SYNCEXT as _,
}
#[derive(Debug, Error)]
pub enum ClockModeError {
    #[error("API returned an error: {0:?}")]
    ApiError(#[from] Error),
    #[error("Unknown clock mask: {0:?}")]
    UnknownMask(#[from] TryFromPrimitiveError<ClockMode>),
}
impl Device {
    pub fn available_clock_modes(&self) -> Result<Vec<ClockMode>, ClockModeError> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_AVAILCLOCKMODES as _, ret_ptr) })?;
        Ok((0..32)
            .map(|i| 1 << i)
            .filter(|x| x & ret > 0)
            .map(|x| x.try_into())
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub fn clock_mode(&self) -> Result<ClockMode, ClockModeError> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_CLOCKMODE as _, ret_ptr) })?;
        Ok(ret.try_into()?)
    }

    pub fn set_clock_mode(&mut self, mode: ClockMode) -> Result<(), ClockModeError> {
        Ok(self.check(unsafe { spcm_dwSetParam_i32(self.0, SPC_CLOCKMODE as _, mode as _) })?)
    }
}

impl Device {
    pub fn reference_clock_frequency(&self) -> Result<i64, Error> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i64(self.0, SPC_REFERENCECLOCK as _, ret_ptr) })?;
        Ok(ret)
    }

    pub fn set_reference_clock_frequency(&mut self, frequency: i64) -> Result<(), Error> {
        self.check(unsafe { spcm_dwSetParam_i64(self.0, SPC_REFERENCECLOCK as _, frequency) })
    }
}

impl Device {
    pub fn clock_out_enabled(&self) -> Result<bool, Error> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, SPC_CLOCKOUT as _, ret_ptr) })?;
        Ok(ret != 0)
    }

    pub fn enable_clock_out(&mut self, enable: bool) -> Result<(), Error> {
        self.check(unsafe { spcm_dwSetParam_i32(self.0, SPC_CLOCKOUT as _, enable as _) })
    }

    pub fn clock_out_frequency(&self) -> Result<i64, Error> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i64(self.0, SPC_CLOCKOUTFREQUENCY as _, ret_ptr) })?;
        Ok(ret)
    }
}

impl Device {
    pub fn sample_rate(&self) -> Result<i64, Error> {
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i64(self.0, SPC_SAMPLERATE as _, ret_ptr) })?;
        Ok(ret)
    }

    pub fn set_sample_rate(&mut self, frequency: i64) -> Result<(), Error> {
        self.check(unsafe { spcm_dwSetParam_i64(self.0, SPC_SAMPLERATE as _, frequency) })
    }
}

impl Device {
    fn register_enable_out(channel: usize) -> i32 {
        assert!(channel < 8);
        [
            SPC_ENABLEOUT0,
            SPC_ENABLEOUT1,
            SPC_ENABLEOUT2,
            SPC_ENABLEOUT3,
            SPC_ENABLEOUT4,
            SPC_ENABLEOUT5,
            SPC_ENABLEOUT6,
            SPC_ENABLEOUT7,
        ][channel] as _
    }

    /// Panics if channel >= 8.
    pub fn channel_out_enabled(&self, channel: usize) -> Result<bool, Error> {
        // Assertion on channel occurs in `register_enable_out`
        let register = Self::register_enable_out(channel);
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, register, ret_ptr) })?;
        Ok(ret != 0)
    }

    /// Panics if channel >= 8.
    pub fn enable_channel_out(&mut self, channel: usize, amplitude: bool) -> Result<(), Error> {
        // Assertion on channel occurs in `register_enable_out`
        let register = Self::register_enable_out(channel);
        self.check(unsafe { spcm_dwSetParam_i32(self.0, register, amplitude as _) })
    }
}

impl Device {
    fn register_amplitude(channel: usize) -> i32 {
        assert!(channel < 16);
        [
            SPC_AMP0, SPC_AMP1, SPC_AMP2, SPC_AMP3, SPC_AMP4, SPC_AMP5, SPC_AMP6, SPC_AMP7,
            SPC_AMP8, SPC_AMP9, SPC_AMP10, SPC_AMP11, SPC_AMP12, SPC_AMP13, SPC_AMP14, SPC_AMP15,
        ][channel] as _
    }

    /// Panics if channel >= 16.
    pub fn channel_amplitude(&self, channel: usize) -> Result<i32, Error> {
        // Assertion on channel occurs in `register_amplitude`
        let register = Self::register_amplitude(channel);
        let mut ret = 0;
        let ret_ptr = (&mut ret) as _;
        self.check(unsafe { spcm_dwGetParam_i32(self.0, register, ret_ptr) })?;
        Ok(ret)
    }

    /// Panics if channel >= 16.
    pub fn set_channel_amplitude(&mut self, channel: usize, enable: i32) -> Result<(), Error> {
        // Assertion on channel occurs in `register_amplitude`
        let register = Self::register_amplitude(channel);
        self.check(unsafe { spcm_dwSetParam_i32(self.0, register, enable) })
    }
}

#[derive(Debug)]
#[repr(i32)]
pub enum M2Command {
    /// hardware reset
    CardReset = M2CMD_CARD_RESET as _,
    /// write setup only
    CardWriteSetup = M2CMD_CARD_WRITESETUP as _,
    /// start of card (including writesetup)
    CardStart = M2CMD_CARD_START as _,
    /// enable trigger engine
    CardEnableTrigger = M2CMD_CARD_ENABLETRIGGER as _,
    /// force trigger
    CardForceTrigger = M2CMD_CARD_FORCETRIGGER as _,
    /// disable trigger engine again (multi or gate)
    CardDisableTrigger = M2CMD_CARD_DISABLETRIGGER as _,
    /// stop run
    CardStop = M2CMD_CARD_STOP as _,
    /// flush fifos to memory
    CardFlushFifo = M2CMD_CARD_FLUSHFIFO as _,
    /// current data in memory is invalidated, next data transfer start will wait until new data is available
    CardInvalidateData = M2CMD_CARD_INVALIDATEDATA as _,
    /// INTERNAL reset command
    CardInternalReset = M2CMD_CARD_INTERNALRESET as _,
    /// stops card and all running transfers
    AllStop = M2CMD_ALL_STOP as _,
    /// wait until pretrigger is full
    CardWaitPrefull = M2CMD_CARD_WAITPREFULL as _,
    /// wait for trigger recognition
    CardWaitTrigger = M2CMD_CARD_WAITTRIGGER as _,
    /// wait for card ready
    CardWaitReady = M2CMD_CARD_WAITREADY as _,
    /// start of DMA transfer for data
    DataStartDma = M2CMD_DATA_STARTDMA as _,
    /// wait for end of data transfer / next block ready
    DataWaitDma = M2CMD_DATA_WAITDMA as _,
    /// abort the data transfer
    DataStopDma = M2CMD_DATA_STOPDMA as _,
    /// transfer data using single access and polling
    DataPoll = M2CMD_DATA_POLL as _,
    /// start of DMA transfer for extra (ABA + timestamp) data
    ExtraStartDma = M2CMD_EXTRA_STARTDMA as _,
    /// wait for end of extra (ABA + timestamp) data transfer / next block ready
    ExtraWaitDma = M2CMD_EXTRA_WAITDMA as _,
    /// abort the extra (ABA + timestamp) data transfer
    ExtraStopDma = M2CMD_EXTRA_STOPDMA as _,
    /// transfer data using single access and polling
    ExtraPoll = M2CMD_EXTRA_POLL as _,
    /// flush incomplete pages from sg list
    DataSgFlush = M2CMD_DATA_SGFLUSH as _,
}
impl Device {
    pub fn execute_command(&mut self, command: M2Command) -> Result<(), Error> {
        self.execute_commands([command])
    }

    pub fn execute_commands(
        &mut self,
        commands: impl IntoIterator<Item = M2Command>,
    ) -> Result<(), Error> {
        let command = commands.into_iter().fold(0, |x, y| x | y as i32);
        self.check(unsafe { spcm_dwSetParam_i32(self.0, SPC_M2CMD as _, command) })
    }
}

#[derive(Debug)]
#[repr(i32)]
pub enum DdsCommand {
    Reset = SPCM_DDS_CMD_RESET as _,
    ExecuteAtTrigger = SPCM_DDS_CMD_EXEC_AT_TRG as _,
    ExecuteNow = SPCM_DDS_CMD_EXEC_NOW as _,
    WriteToCard = SPCM_DDS_CMD_WRITE_TO_CARD as _,
    NoNopFill = SPCM_DDS_CMD_NO_NOP_FILL as _,
}
impl Device {
    pub fn execute_dds_command(&mut self, command: DdsCommand) -> Result<(), Error> {
        self.check(unsafe { spcm_dwSetParam_i32(self.0, SPC_DDS_CMD as _, command as _) })
    }
}

#[derive(Clone, Copy)]
pub struct DdsCore<'a>(&'a Device, usize);
pub struct DdsCoreMut<'a>(DdsCore<'a>);
impl<'a> std::ops::Deref for DdsCoreMut<'a> {
    type Target = DdsCore<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Device {
    /// Panics if index >= 23.
    pub fn dds_core(&self, index: usize) -> DdsCore {
        assert!(index < 23);
        DdsCore(self, index)
    }

    /// Panics if index >= 23.
    pub fn dds_core_mut(&mut self, index: usize) -> DdsCoreMut {
        DdsCoreMut(self.dds_core(index))
    }
}
impl DdsCore<'_> {
    fn amplitude_register(self) -> i32 {
        [
            SPC_DDS_CORE0_AMP,
            SPC_DDS_CORE1_AMP,
            SPC_DDS_CORE2_AMP,
            SPC_DDS_CORE3_AMP,
            SPC_DDS_CORE4_AMP,
            SPC_DDS_CORE5_AMP,
            SPC_DDS_CORE6_AMP,
            SPC_DDS_CORE7_AMP,
            SPC_DDS_CORE8_AMP,
            SPC_DDS_CORE9_AMP,
            SPC_DDS_CORE10_AMP,
            SPC_DDS_CORE11_AMP,
            SPC_DDS_CORE12_AMP,
            SPC_DDS_CORE13_AMP,
            SPC_DDS_CORE14_AMP,
            SPC_DDS_CORE15_AMP,
            SPC_DDS_CORE16_AMP,
            SPC_DDS_CORE17_AMP,
            SPC_DDS_CORE18_AMP,
            SPC_DDS_CORE19_AMP,
            SPC_DDS_CORE20_AMP,
            SPC_DDS_CORE21_AMP,
            SPC_DDS_CORE22_AMP,
        ][self.1] as i32
    }
    pub fn amplitude(self) -> Result<f64, Error> {
        let register = self.amplitude_register();
        let mut ret = 0.;
        let ret_ptr = (&mut ret) as _;
        self.0
            .check(unsafe { spcm_dwGetParam_d64(self.0 .0, register, ret_ptr) })?;
        Ok(ret)
    }
    fn frequency_register(self) -> i32 {
        [
            SPC_DDS_CORE0_FREQ,
            SPC_DDS_CORE1_FREQ,
            SPC_DDS_CORE2_FREQ,
            SPC_DDS_CORE3_FREQ,
            SPC_DDS_CORE4_FREQ,
            SPC_DDS_CORE5_FREQ,
            SPC_DDS_CORE6_FREQ,
            SPC_DDS_CORE7_FREQ,
            SPC_DDS_CORE8_FREQ,
            SPC_DDS_CORE9_FREQ,
            SPC_DDS_CORE10_FREQ,
            SPC_DDS_CORE11_FREQ,
            SPC_DDS_CORE12_FREQ,
            SPC_DDS_CORE13_FREQ,
            SPC_DDS_CORE14_FREQ,
            SPC_DDS_CORE15_FREQ,
            SPC_DDS_CORE16_FREQ,
            SPC_DDS_CORE17_FREQ,
            SPC_DDS_CORE18_FREQ,
            SPC_DDS_CORE19_FREQ,
            SPC_DDS_CORE20_FREQ,
            SPC_DDS_CORE21_FREQ,
            SPC_DDS_CORE22_FREQ,
        ][self.1] as i32
    }
    pub fn frequency(self) -> Result<f64, Error> {
        let register = self.frequency_register();
        let mut ret = 0.;
        let ret_ptr = (&mut ret) as _;
        self.0
            .check(unsafe { spcm_dwGetParam_d64(self.0 .0, register, ret_ptr) })?;
        Ok(ret)
    }
    fn phase_register(self) -> i32 {
        [
            SPC_DDS_CORE0_PHASE,
            SPC_DDS_CORE1_PHASE,
            SPC_DDS_CORE2_PHASE,
            SPC_DDS_CORE3_PHASE,
            SPC_DDS_CORE4_PHASE,
            SPC_DDS_CORE5_PHASE,
            SPC_DDS_CORE6_PHASE,
            SPC_DDS_CORE7_PHASE,
            SPC_DDS_CORE8_PHASE,
            SPC_DDS_CORE9_PHASE,
            SPC_DDS_CORE10_PHASE,
            SPC_DDS_CORE11_PHASE,
            SPC_DDS_CORE12_PHASE,
            SPC_DDS_CORE13_PHASE,
            SPC_DDS_CORE14_PHASE,
            SPC_DDS_CORE15_PHASE,
            SPC_DDS_CORE16_PHASE,
            SPC_DDS_CORE17_PHASE,
            SPC_DDS_CORE18_PHASE,
            SPC_DDS_CORE19_PHASE,
            SPC_DDS_CORE20_PHASE,
            SPC_DDS_CORE21_PHASE,
            SPC_DDS_CORE22_PHASE,
        ][self.1] as i32
    }
    pub fn phase(self) -> Result<f64, Error> {
        let register = self.phase_register();
        let mut ret = 0.;
        let ret_ptr = (&mut ret) as _;
        self.0
            .check(unsafe { spcm_dwGetParam_d64(self.0 .0, register, ret_ptr) })?;
        Ok(ret)
    }
    fn frequency_slope_register(self) -> i32 {
        [
            SPC_DDS_CORE0_FREQ_SLOPE,
            SPC_DDS_CORE1_FREQ_SLOPE,
            SPC_DDS_CORE2_FREQ_SLOPE,
            SPC_DDS_CORE3_FREQ_SLOPE,
            SPC_DDS_CORE4_FREQ_SLOPE,
            SPC_DDS_CORE5_FREQ_SLOPE,
            SPC_DDS_CORE6_FREQ_SLOPE,
            SPC_DDS_CORE7_FREQ_SLOPE,
            SPC_DDS_CORE8_FREQ_SLOPE,
            SPC_DDS_CORE9_FREQ_SLOPE,
            SPC_DDS_CORE10_FREQ_SLOPE,
            SPC_DDS_CORE11_FREQ_SLOPE,
            SPC_DDS_CORE12_FREQ_SLOPE,
            SPC_DDS_CORE13_FREQ_SLOPE,
            SPC_DDS_CORE14_FREQ_SLOPE,
            SPC_DDS_CORE15_FREQ_SLOPE,
            SPC_DDS_CORE16_FREQ_SLOPE,
            SPC_DDS_CORE17_FREQ_SLOPE,
            SPC_DDS_CORE18_FREQ_SLOPE,
            SPC_DDS_CORE19_FREQ_SLOPE,
            SPC_DDS_CORE20_FREQ_SLOPE,
            SPC_DDS_CORE21_FREQ_SLOPE,
            SPC_DDS_CORE22_FREQ_SLOPE,
        ][self.1] as i32
    }
    pub fn frequency_slope(self) -> Result<f64, Error> {
        let register = self.frequency_slope_register();
        let mut ret = 0.;
        let ret_ptr = (&mut ret) as _;
        self.0
            .check(unsafe { spcm_dwGetParam_d64(self.0 .0, register, ret_ptr) })?;
        Ok(ret)
    }
    fn amplitude_slope_register(self) -> i32 {
        [
            SPC_DDS_CORE0_AMP_SLOPE,
            SPC_DDS_CORE1_AMP_SLOPE,
            SPC_DDS_CORE2_AMP_SLOPE,
            SPC_DDS_CORE3_AMP_SLOPE,
            SPC_DDS_CORE4_AMP_SLOPE,
            SPC_DDS_CORE5_AMP_SLOPE,
            SPC_DDS_CORE6_AMP_SLOPE,
            SPC_DDS_CORE7_AMP_SLOPE,
            SPC_DDS_CORE8_AMP_SLOPE,
            SPC_DDS_CORE9_AMP_SLOPE,
            SPC_DDS_CORE10_AMP_SLOPE,
            SPC_DDS_CORE11_AMP_SLOPE,
            SPC_DDS_CORE12_AMP_SLOPE,
            SPC_DDS_CORE13_AMP_SLOPE,
            SPC_DDS_CORE14_AMP_SLOPE,
            SPC_DDS_CORE15_AMP_SLOPE,
            SPC_DDS_CORE16_AMP_SLOPE,
            SPC_DDS_CORE17_AMP_SLOPE,
            SPC_DDS_CORE18_AMP_SLOPE,
            SPC_DDS_CORE19_AMP_SLOPE,
            SPC_DDS_CORE20_AMP_SLOPE,
            SPC_DDS_CORE21_AMP_SLOPE,
            SPC_DDS_CORE22_AMP_SLOPE,
        ][self.1] as i32
    }
    pub fn amplitude_slope(self) -> Result<f64, Error> {
        let register = self.amplitude_slope_register();
        let mut ret = 0.;
        let ret_ptr = (&mut ret) as _;
        self.0
            .check(unsafe { spcm_dwGetParam_d64(self.0 .0, register, ret_ptr) })?;
        Ok(ret)
    }
}
impl DdsCoreMut<'_> {
    pub fn set_amplitude(&mut self, amplitude: f64) -> Result<(), Error> {
        let register = self.amplitude_register();
        (self.0 .0).check(unsafe { spcm_dwSetParam_d64(self.0 .0 .0, register, amplitude) })
    }
    pub fn set_frequency(&mut self, frequency: f64) -> Result<(), Error> {
        let register = self.frequency_register();
        (self.0 .0).check(unsafe { spcm_dwSetParam_d64(self.0 .0 .0, register, frequency) })
    }
    pub fn set_phase(&mut self, phase: f64) -> Result<(), Error> {
        let register = self.phase_register();
        (self.0 .0).check(unsafe { spcm_dwSetParam_d64(self.0 .0 .0, register, phase) })
    }
    pub fn set_frequency_slope(&mut self, frequency_slope: f64) -> Result<(), Error> {
        let register = self.frequency_slope_register();
        (self.0 .0).check(unsafe { spcm_dwSetParam_d64(self.0 .0 .0, register, frequency_slope) })
    }
    pub fn set_amplitude_slope(&mut self, amplitude_slope: f64) -> Result<(), Error> {
        let register = self.amplitude_slope_register();
        (self.0 .0).check(unsafe { spcm_dwSetParam_d64(self.0 .0 .0, register, amplitude_slope) })
    }
}
