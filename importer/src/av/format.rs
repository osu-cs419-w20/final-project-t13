#![allow(non_snake_case)]

use std::cell::Cell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;

use super::error::AVError;
use super::{error, utils};

pub enum Format {
    FLAC,
    MP3,
}

pub struct AVFormatContext {
    ctx: *mut ffmpeg_sys::AVFormatContext,
    stream_info_read: Cell<bool>,
}

impl AVFormatContext {
    pub fn open<P: AsRef<Path>>(p: P) -> super::Result<AVFormatContext> {
        let path_str = p.as_ref().to_str().ok_or(AVError::EncodingError(None))?;
        let path = CString::new(path_str)?;
        let mut ctx = Self::alloc()?;

        let fmt: *mut ffmpeg_sys::AVInputFormat = ptr::null_mut();
        let mut opts: *mut ffmpeg_sys::AVDictionary = ptr::null_mut();

        unsafe {
            let open_res = ffmpeg_sys::avformat_open_input(&mut ctx, path.as_ptr(), fmt, &mut opts);
            if open_res < 0 {
                let msg = match error::av_error_to_string(open_res)? {
                    Ok(msg) => msg,
                    Err(res) => format!("failed to convert error code {} to string; received error code {}", open_res, res),
                };
                return Err(AVError::AVLibraryError(msg));
            }
        }

        let context = AVFormatContext {
            ctx,
            stream_info_read: Cell::new(false),
        };

        Ok(context)
    }

    fn find_stream_info(&self) -> super::Result<()> {
        if !self.stream_info_read.get() {
            let ret = unsafe { ffmpeg_sys::avformat_find_stream_info(self.ctx, ptr::null_mut()) };
            if ret < 0 {
                let msg = match error::av_error_to_string(ret)? {
                    Ok(msg) => msg,
                    Err(res) => format!("failed to convert error code {} to string; received error code {}", ret, res),
                };
                return Err(AVError::AVLibraryError(msg));
            }
            self.stream_info_read.replace(true);
        }
        Ok(())
    }

    fn alloc() -> super::Result<*mut ffmpeg_sys::AVFormatContext> {
        let ctx = unsafe { ffmpeg_sys::avformat_alloc_context() };
        if ctx == ptr::null_mut() {
            Err(AVError::NullPointer("AVFormatContext".to_string()))
        } else {
            Ok(ctx)
        }
    }

    pub fn close(self) {}

    pub fn metadata<'a, 'b>(&'a self) -> super::Result<HashMap<&'b str, &'b str>> {
        unsafe { utils::av_dict_as_hash((*self.ctx).metadata) }
    }

    pub fn determine_format(&self) -> super::Result<Format> {
        let mut stream = ptr::null();
        let mut params = ptr::null();
        let mut candidates = Vec::new();

        unsafe {
            self.find_stream_info()?;
            for i in 0..(*self.ctx).nb_streams {
                stream = *(*self.ctx).streams.offset(i as isize);
                params = (*stream).codecpar;

                if (*params).codec_type != ffmpeg_sys::AVMediaType_AVMEDIA_TYPE_AUDIO {
                    continue;
                }

                match (*params).codec_id {
                    ffmpeg_sys::AVCodecID_AV_CODEC_ID_MP3 => candidates.push(Format::MP3),
                    ffmpeg_sys::AVCodecID_AV_CODEC_ID_FLAC => candidates.push(Format::FLAC),
                    _ => {}
                }
            }
        }

        // What should be done if there are multiple audio streams?
        if candidates.len() == 1 {
            Ok(candidates.remove(0))
        } else {
            Err(AVError::UnknownFormat)
        }
    }

    pub fn bit_rate(&self) -> i64 {
        unsafe { (*self.ctx).bit_rate / 1000 }
    }

    pub fn duration(&self) -> i64 {
        unsafe { (*self.ctx).duration / (ffmpeg_sys::AV_TIME_BASE as i64) }
    }
}

impl Drop for AVFormatContext {
    fn drop(&mut self) {
        if self.ctx != ptr::null_mut() {
            unsafe { ffmpeg_sys::avformat_close_input(&mut self.ctx); }
        }
    }
}
