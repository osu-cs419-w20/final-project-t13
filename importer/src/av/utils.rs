use std::collections::HashMap;
use std::ffi::CStr;
use std::ptr;
use std::os::raw;

pub(super) fn av_dict_as_hash<'a>(dict: *const ffmpeg_sys::AVDictionary) -> super::Result<HashMap<&'a str, &'a str>> {
    let mut hash = HashMap::new();

    unsafe {
        let key = CStr::from_bytes_with_nul_unchecked(b"");
        let flags = ffmpeg_sys::AV_DICT_IGNORE_SUFFIX as i32;
        let mut entry = ffmpeg_sys::av_dict_get(dict, key.as_ptr(), ptr::null(), flags);
        while entry != ptr::null_mut() {
            let k = char_ptr_to_str((*entry).key)?;
            let v = char_ptr_to_str((*entry).value)?;
            hash.insert(k, v);
            entry = ffmpeg_sys::av_dict_get(dict, key.as_ptr(), entry, flags);
        }
    }

    Ok(hash)
}

pub(super) fn char_ptr_to_str<'a>(s: *const raw::c_char) -> super::Result<&'a str> {
    unsafe { CStr::from_ptr(s).to_str().map_err(|e| e.into()) }
}
