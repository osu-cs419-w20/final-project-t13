use std::{ffi, str};
use std::os::raw;

use super::utils;

#[derive(Debug)]
pub enum AVError {
    EncodingError(Option<str::Utf8Error>),
    PathNulByteError(ffi::NulError),
    AllocationError(&'static str),
    AVLibraryError(String),
    NullPointer(String),
    UnknownFormat,
}

impl std::error::Error for AVError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use AVError::*;
        match self {
            EncodingError(Some(e)) => Some(e),
            PathNulByteError(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for AVError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use AVError::*;
        match self {
            EncodingError(_) => write!(fmt, "encoding error converting raw string"),
            PathNulByteError(_) => write!(fmt, "provided path contains a nul byte"),
            AllocationError(t) => write!(fmt, "failed to allocate memory for {}", t),
            AVLibraryError(e) => write!(fmt, "av library error: {}", e),
            NullPointer(_) => write!(fmt, "unexpected null pointer"),
            UnknownFormat => write!(fmt, "unknown format"),
        }
    }
}

impl From<ffi::NulError> for AVError {
    fn from(e: ffi::NulError) -> Self {
        AVError::PathNulByteError(e)
    }
}

impl From<str::Utf8Error> for AVError {
    fn from(e: str::Utf8Error) -> Self {
        AVError::EncodingError(Some(e))
    }
}

pub(super) fn av_error_to_string(code: i32) -> super::Result<Result<String, i32>> {
    const BUF_SIZE: usize = ffmpeg_sys::AV_ERROR_MAX_STRING_SIZE as usize;
    let mut buf = [0 as raw::c_char; BUF_SIZE];
    let ptr = &mut buf[0];
    let res = unsafe { ffmpeg_sys::av_strerror(code, ptr, BUF_SIZE as u64) };

    if res < 0 {
        return Ok(Err(res));
    }

    Ok(Ok(utils::char_ptr_to_str(ptr)?.to_string()))
}
