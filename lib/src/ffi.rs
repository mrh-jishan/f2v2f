//! C FFI bindings for f2v2f
//!
//! This module provides C-compatible function signatures that can be called
//! from Python, TypeScript/Node.js, and other languages via FFI.

use crate::config::{EncodeConfig, DecodeConfig};
use crate::encoder::Encoder;
use crate::decoder::Decoder;
use crate::video_composer::VideoComposer;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LAST_ERROR: Mutex<Option<String>> = Mutex::new(None);
}

fn set_last_error(err: String) {
    if let Ok(mut guard) = LAST_ERROR.lock() {
        *guard = Some(err);
    }
}

fn clear_last_error() {
    if let Ok(mut guard) = LAST_ERROR.lock() {
        *guard = None;
    }
}

/// Opaque handle for ongoing encode operations
pub struct EncodeHandle {
    encoder: Encoder,
    config: EncodeConfig,
}

/// Opaque handle for ongoing decode operations
pub struct DecodeHandle {
    decoder: Decoder,
}

lazy_static! {
    static ref ENCODE_HANDLES: Mutex<Vec<Box<EncodeHandle>>> = Mutex::new(Vec::new());
    static ref DECODE_HANDLES: Mutex<Vec<Box<DecodeHandle>>> = Mutex::new(Vec::new());
    // Global Tokio runtime - created once and reused for all FFI calls
    // This prevents runtime from being destroyed while async operations are in progress
    static ref TOKIO_RUNTIME: tokio::runtime::Runtime = {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create global Tokio runtime")
    };
}

/// Error codes for C API
#[repr(C)]
pub enum F2V2FErrorCode {
    Success = 0,
    InvalidInput = 1,
    IoError = 2,
    EncodingError = 3,
    DecodingError = 4,
    ConfigError = 5,
    OperationInProgress = 6,
    InvalidHandle = 7,
    Unknown = 255,
}

/// Progress callback function signature
pub type ProgressCallback = extern "C" fn(u64, u64, *const c_char);

/// Callback for operation completion
pub type CompletionCallback = extern "C" fn(i32, *const c_char);

/// Initialize the library (call once at startup)
#[no_mangle]
pub extern "C" fn f2v2f_init() -> i32 {
    // Initialize logging
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init();
    
    F2V2FErrorCode::Success as i32
}

/// Get the last error message
/// Returns a pointer to a null-terminated string. The caller must free it.
#[no_mangle]
pub extern "C" fn f2v2f_get_last_error() -> *mut c_char {
    if let Ok(guard) = LAST_ERROR.lock() {
        match guard.as_ref() {
            Some(err) => {
                match CString::new(err.as_str()) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                }
            }
            None => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}

/// Free a string returned by f2v2f_get_last_error
#[no_mangle]
pub extern "C" fn f2v2f_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

/// Create an encoding context
///
/// # Safety
/// - `input_path` must be a valid null-terminated UTF-8 string
/// - Returned handle must be freed with `f2v2f_encode_free`
#[no_mangle]
pub extern "C" fn f2v2f_encode_create(
    width: u32,
    height: u32,
    fps: u32,
    chunk_size: usize,
) -> *mut EncodeHandle {
    let config = EncodeConfig {
        width,
        height,
        fps,
        chunk_size,
        art_style: "geometric".to_string(),
        num_threads: num_cpus::get(),
        buffer_size: 1024 * 1024,
        use_compression: true,
        compression_level: 11,
    };

    if let Err(_) = config.validate() {
        return std::ptr::null_mut();
    }

    match Encoder::new(config.clone()) {
        Ok(encoder) => {
            let handle = Box::new(EncodeHandle { encoder, config });
            Box::into_raw(handle)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Encode a file to video
///
/// # Safety
/// - `handle` must be a valid pointer from `f2v2f_encode_create`
/// - `input_path` and `output_path` must be valid null-terminated UTF-8 strings
/// - `encoded_size_out` must be a valid pointer to u64 (nullable)
/// - `progress_callback` must be null (callbacks not yet supported)
#[no_mangle]
pub extern "C" fn f2v2f_encode_file(
    handle: *mut EncodeHandle,
    input_path: *const c_char,
    output_path: *const c_char,
    encoded_size_out: *mut u64,
    progress_callback: Option<ProgressCallback>,
) -> i32 {
    if handle.is_null() {
        return F2V2FErrorCode::InvalidHandle as i32;
    }

    let input_cstr = unsafe { CStr::from_ptr(input_path) };
    let output_cstr = unsafe { CStr::from_ptr(output_path) };

    let input_path_str = match input_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return F2V2FErrorCode::InvalidInput as i32,
    };

    let output_path_str = match output_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return F2V2FErrorCode::InvalidInput as i32,
    };

    let handle_ref = unsafe { &*handle };

    // IMPORTANT: Call blocking methods directly - NO async runtime!
    // This prevents SIGBUS crashes from Tokio runtime in cgo context
    
    // Encode the file data (blocking)
    let (info, compressed_data) = match handle_ref.encoder.encode_blocking(input_path_str) {
        Ok(result) => result,
        Err(e) => {
            set_last_error(format!("{}", e));
            return F2V2FErrorCode::EncodingError as i32;
        }
    };

    // Set encoded size output if provided
    if !encoded_size_out.is_null() {
        unsafe {
            *encoded_size_out = info.encoded_size;
        }
    }

    // Call progress callback with encoding progress (not implemented)
    if let Some(_callback) = progress_callback {
        // Callbacks not yet supported in FFI layer
    }

    // Create video from file data using optimized chunk size (BLOCKING)
    let composer = VideoComposer::new(
        handle_ref.config.width,
        handle_ref.config.height,
        handle_ref.config.fps,
    );

    match composer.compose_from_file_data_blocking_with_original(
        compressed_data,
        info.chunk_size,
        info.original_file_size,  // Pass original file size for metadata
        output_path_str,
    ) {
        Ok(_) => {
            clear_last_error();
            F2V2FErrorCode::Success as i32
        },
        Err(e) => {
            set_last_error(format!("{}", e));
            F2V2FErrorCode::EncodingError as i32
        },
    }
}

/// Free an encoding handle
///
/// # Safety
/// - `handle` must be a valid pointer from `f2v2f_encode_create`
/// - Do not use handle after calling this
#[no_mangle]
pub extern "C" fn f2v2f_encode_free(handle: *mut EncodeHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        }
    }
}

/// Create a decoding context
#[no_mangle]
pub extern "C" fn f2v2f_decode_create() -> *mut DecodeHandle {
    let config = DecodeConfig::default();

    if let Err(_) = config.validate() {
        return std::ptr::null_mut();
    }

    match Decoder::new(config) {
        Ok(decoder) => {
            let handle = Box::new(DecodeHandle { decoder });
            Box::into_raw(handle)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Create a decoding context with parameters
#[no_mangle]
pub extern "C" fn f2v2f_decode_create_with_params(
    width: u32,
    height: u32,
    chunk_size: usize,
) -> *mut DecodeHandle {
    let config = DecodeConfig {
        width,
        height,
        chunk_size,
        ..DecodeConfig::default()
    };

    if let Err(_) = config.validate() {
        return std::ptr::null_mut();
    }

    match Decoder::new(config) {
        Ok(decoder) => {
            let handle = Box::new(DecodeHandle { decoder });
            Box::into_raw(handle)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Decode a video back to a file
///
/// # Safety
/// - `handle` must be a valid pointer from `f2v2f_decode_create`
/// - `input_path` and `output_path` must be valid null-terminated UTF-8 strings
#[no_mangle]
pub extern "C" fn f2v2f_decode_file(
    handle: *mut DecodeHandle,
    input_path: *const c_char,
    output_path: *const c_char,
    progress_callback: Option<ProgressCallback>,
) -> i32 {
    if handle.is_null() {
        return F2V2FErrorCode::InvalidHandle as i32;
    }

    let input_cstr = unsafe { CStr::from_ptr(input_path) };
    let output_cstr = unsafe { CStr::from_ptr(output_path) };

    let input_path_str = match input_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return F2V2FErrorCode::InvalidInput as i32,
    };

    let output_path_str = match output_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return F2V2FErrorCode::InvalidInput as i32,
    };

    let handle_ref = unsafe { &*handle };

    // Use the global Tokio runtime for consistency
    match TOKIO_RUNTIME.block_on(handle_ref.decoder.decode(input_path_str, output_path_str)) {
        Ok(info) => {
            if let Some(callback) = progress_callback {
                let status_msg = CString::new(format!(
                    "Decoded {} bytes with checksum {}",
                    info.extracted_size, info.checksum
                ))
                .unwrap();
                callback(info.extracted_size, 0, status_msg.as_ptr());
            }
            clear_last_error();
            F2V2FErrorCode::Success as i32
        }
        Err(e) => {
            set_last_error(format!("{}", e));
            F2V2FErrorCode::DecodingError as i32
        },
    }
}

/// Free a decoding handle
///
/// # Safety
/// - `handle` must be a valid pointer from `f2v2f_decode_create`
/// - Do not use handle after calling this
#[no_mangle]
pub extern "C" fn f2v2f_decode_free(handle: *mut DecodeHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        }
    }
}

/// Get version string
///
/// Returns: Static string with version info
#[no_mangle]
pub extern "C" fn f2v2f_version() -> *const c_char {
    c"f2v2f v0.1.0".as_ptr() as *const c_char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let result = f2v2f_init();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_version() {
        let version = unsafe { CStr::from_ptr(f2v2f_version()).to_str().unwrap() };
        assert!(version.contains("f2v2f"));
    }
}
