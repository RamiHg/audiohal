use lazy_static::lazy_static;
use libportaudio_sys as ffi;
use std::sync::{Mutex, MutexGuard};

mod error;
mod host;
mod stream_options;

pub mod device;

// Helpful exports.
pub use device::Device;
pub use host::Host;

lazy_static! {
    static ref GLOBAL_LOCK: Mutex<()> = Mutex::new(());
}

type LockGuard = MutexGuard<'static, ()>;

fn global_lock() -> LockGuard {
    GLOBAL_LOCK
        .lock()
        .expect("A PortAudio host thread has panicked. Aborting.")
}

impl std::convert::TryFrom<crate::Format> for ffi::PaSampleFormat {
    type Error = crate::error::Error;
    fn try_from(format: crate::Format) -> crate::error::Result<ffi::PaSampleFormat> {
        use crate::error::Error;
        use crate::Format::*;
        use ffi::PaSampleFormat;
        Ok(match format {
            F32 => PaSampleFormat::paFloat32,
            I32 => PaSampleFormat::paInt32,
            I24 => PaSampleFormat::paInt24,
            I16 => PaSampleFormat::paInt16,
            I8 => PaSampleFormat::paInt8,
            U8 => PaSampleFormat::paUInt8,
            _ => return Err(Error::IncompatibleFormat(format)),
        })
    }
}