use std::{
    ffi::{CString, NulError},
    marker::PhantomData,
};

use chrono::{DateTime, NaiveDateTime};
use connection_mode::{ConnectionMode, FreeRun, TriggerAsync, TriggerMode, TriggerSync};
use dl950acqapi_sys::{
    ScCloseInstrument, ScExit, ScGetAcqCount, ScGetAcqData, ScGetAcqDataLength, ScGetLatchAcqCount,
    ScInit, ScLatchData, ScOpenInstrument, ScSetAcqCount, ScStart, ScStop, SC_ERROR, SC_SUCCESS,
    SC_WIRE_HISLIP, SC_WIRE_USBTMC, SC_WIRE_VISAUSB, SC_WIRE_VXI11,
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
    pub fn open_free_run(
        &self,
        wire_type: WireType,
        address: &str,
    ) -> Result<Handle<FreeRun>, OpenError> {
        Handle::open(wire_type, address)
    }

    pub fn open_trigger_sync(
        &self,
        wire_type: WireType,
        address: &str,
    ) -> Result<Handle<TriggerSync>, OpenError> {
        Handle::open(wire_type, address)
    }

    pub fn open_trigger_async(
        &self,
        wire_type: WireType,
        address: &str,
    ) -> Result<Handle<TriggerAsync>, OpenError> {
        Handle::open(wire_type, address)
    }
}

pub struct Handle<T>(i32, PhantomData<fn() -> T>);
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
pub mod connection_mode {
    use sealed::sealed;

    use dl950acqapi_sys::{SC_FREERUN, SC_TRIGGER, SC_TRIGGER_ASYNC};
    use std::convert::Infallible;

    pub struct FreeRun(Infallible);
    /// Trigger mode (synchronized).
    /// Triggers only after data transmission is confirmed.
    pub struct TriggerSync(Infallible);
    /// Trigger mode (not synchronized).
    /// Triggers regardless of data transmission.
    pub struct TriggerAsync(Infallible);

    #[sealed]
    pub trait ConnectionMode {
        const MODE_NUM: u32;
    }
    #[sealed]
    impl ConnectionMode for FreeRun {
        const MODE_NUM: u32 = SC_FREERUN;
    }
    #[sealed]
    impl ConnectionMode for TriggerSync {
        const MODE_NUM: u32 = SC_TRIGGER;
    }
    #[sealed]
    impl ConnectionMode for TriggerAsync {
        const MODE_NUM: u32 = SC_TRIGGER_ASYNC;
    }

    #[sealed]
    pub trait TriggerMode {}
    #[sealed]
    impl TriggerMode for TriggerSync {}
    #[sealed]
    impl TriggerMode for TriggerAsync {}
}

#[derive(Debug, Error)]
pub enum OpenError {
    #[error("The provided address contained null characters: {0}")]
    AddressContainedNul(#[from] NulError),
    #[error("The API returned an error: {0}")]
    ApiError(#[from] Error),
}

impl<T: ConnectionMode> Handle<T> {
    fn open(wire_type: WireType, address: &str) -> Result<Handle<T>, OpenError> {
        let wire = match wire_type {
            WireType::UsbTmc => SC_WIRE_USBTMC,
            WireType::VisaUsb => SC_WIRE_VISAUSB,
            WireType::Vxi11 => SC_WIRE_VXI11,
            WireType::HiSlip => SC_WIRE_HISLIP,
        } as i32;
        let address = CString::new(address)?;
        let address = address.as_ptr() as _;
        let mode = T::MODE_NUM as i32;
        let mut handle = 0;
        let handle_ptr = &mut handle as *mut i32;
        check(unsafe { ScOpenInstrument(wire, address, mode, handle_ptr) })?;
        Ok(Handle(handle, PhantomData))
    }
}
impl<T> Handle<T> {
    pub fn close(self) -> Result<(), Error> {
        self.close_impl()
    }

    fn close_impl(&self) -> Result<(), Error> {
        check(unsafe { ScCloseInstrument(self.0) })
    }
}
impl<T> Drop for Handle<T> {
    fn drop(&mut self) {
        if let Err(e) = self.close_impl() {
            e.report_drop_error();
        }
    }
}

impl<T> Handle<T> {
    pub fn start(&self) -> Result<(), Error> {
        check(unsafe { ScStart(self.0) })
    }

    pub fn stop(&self) -> Result<(), Error> {
        check(unsafe { ScStop(self.0) })
    }

    pub fn latch_data(&self) -> Result<(), Error> {
        check(unsafe { ScLatchData(self.0) })
    }
}

#[derive(Clone, Copy)]
pub struct ChannelNumber {
    pub channel: u8,
    pub sub_channel: u8,
}
impl ChannelNumber {
    pub fn new(channel: u8, sub_channel: u8) -> Self {
        Self {
            channel,
            sub_channel,
        }
    }
}

#[derive(Debug)]
pub struct ReadTriggeredWaveformReturn {
    pub received_len: usize,
    pub completed: bool,
    pub timestamp: NaiveDateTime,
}

impl<T: TriggerMode> Handle<T> {
    pub fn read_triggered_waveform(
        &self,
        channel: ChannelNumber,
        buffer: &mut [u8],
    ) -> Result<ReadTriggeredWaveformReturn, Error> {
        let mut received_len = 0;
        let mut end_flag = 0;
        let mut time_secs = 0u32;
        let mut time_nanos = 0u32;
        {
            let buffer_ptr = buffer.as_mut_ptr() as *mut i8;
            let buffer_len = buffer.len() as _;
            let received_len = &mut received_len as *mut _;
            let end_flag = &mut end_flag as *mut _;
            let time_secs = &mut time_secs as *mut _;
            let time_nanos = &mut time_nanos as *mut _;
            check(unsafe {
                ScGetAcqData(
                    self.0,
                    channel.channel as _,
                    channel.sub_channel as _,
                    buffer_ptr,
                    buffer_len,
                    received_len,
                    end_flag,
                    time_secs,
                    time_nanos,
                )
            })?;
        }
        // We assume that the library is correct (nanoseconds part is never out of range).
        let timestamp = DateTime::from_timestamp(time_secs as i64, time_nanos)
            .expect("Nanosecond part is out of range")
            .naive_utc(); // It seems that the timestamp is offset by 9:00, implying it's not true
                          // UNIX time.
        Ok(ReadTriggeredWaveformReturn {
            received_len: received_len as _,
            completed: end_flag != 0,
            timestamp,
        })
    }

    pub fn triggered_samples_len(&self, channel: ChannelNumber) -> Result<usize, Error> {
        let mut ret = 0i64;
        {
            let ret = &mut ret as *mut _;
            check(unsafe {
                ScGetAcqDataLength(self.0, channel.channel as _, channel.sub_channel as _, ret)
            })?;
        }
        // Returned value is between 0 and 999_999_999, so casting is safe.
        Ok(ret as usize)
    }

    pub fn latched_acquisition_count(&self) -> Result<i64, Error> {
        let mut ret = 0i64;
        {
            let ret = &mut ret as *mut _;
            check(unsafe { ScGetLatchAcqCount(self.0, ret) })?;
        }
        // Returned value must be not very large... at least below the memory size.
        Ok(ret)
    }

    pub fn acquisition_index(&self) -> Result<i64, Error> {
        let mut ret = 0i64;
        {
            let ret = &mut ret as *mut _;
            check(unsafe { ScGetAcqCount(self.0, ret) })?;
        }
        // Same as latched_acquisition_count
        Ok(ret)
    }

    pub fn set_acquisition_index(&self, index: i64) -> Result<(), Error> {
        check(unsafe { ScSetAcqCount(self.0, index) })
    }
}
