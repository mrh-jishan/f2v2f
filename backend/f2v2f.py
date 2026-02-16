"""
f2v2f Python bindings using ctypes

Low-level ctypes wrapper for the f2v2f Rust library.
For most use cases, prefer using the high-level F2V2F class instead.
"""

import ctypes
import os
import sys
from pathlib import Path
from typing import Optional, Callable

# Determine platform and library name
platform = sys.platform
if platform == "darwin":
    libname = "libf2v2f.dylib"
elif platform == "win32":
    libname = "f2v2f.dll"
else:
    libname = "libf2v2f.so"

# Try to find the library in common locations
lib_paths = [
    Path(__file__).parent.parent / "lib" / "target" / "release" / libname,  # Current structure: f2v2f/lib/target/release
    Path(__file__).parent.parent / "lib" / "target" / "debug" / libname,    # Current structure: f2v2f/lib/target/debug
    Path(__file__).parent.parent / "target" / "release" / libname,          # Old structure (fallback)
    Path(__file__).parent.parent / "target" / "debug" / libname,            # Old structure (fallback)
    Path("/usr/local/lib") / libname,
    Path("/usr/lib") / libname,
    Path(libname),
]

_lib = None
for lib_path in lib_paths:
    try:
        if lib_path.exists():
            _lib = ctypes.CDLL(str(lib_path))
            break
    except OSError:
        continue

if _lib is None:
    raise RuntimeError(
        f"Could not find f2v2f library. Searched:\n" + 
        "\n".join(str(p) for p in lib_paths)
    )

# Error codes
class ErrorCode:
    SUCCESS = 0
    INVALID_INPUT = 1
    IO_ERROR = 2
    ENCODING_ERROR = 3
    DECODING_ERROR = 4
    CONFIG_ERROR = 5
    OPERATION_IN_PROGRESS = 6
    INVALID_HANDLE = 7
    UNKNOWN = 255

# Define function signatures
_lib.f2v2f_init.argtypes = []
_lib.f2v2f_init.restype = ctypes.c_int

_lib.f2v2f_encode_create.argtypes = [
    ctypes.c_uint32,  # width
    ctypes.c_uint32,  # height
    ctypes.c_uint32,  # fps
    ctypes.c_size_t,  # chunk_size
]
_lib.f2v2f_encode_create.restype = ctypes.c_void_p

_lib.f2v2f_encode_file.argtypes = [
    ctypes.c_void_p,  # handle
    ctypes.c_char_p,  # input_path
    ctypes.c_char_p,  # output_path
    ctypes.c_void_p,  # progress_callback
]
_lib.f2v2f_encode_file.restype = ctypes.c_int

_lib.f2v2f_encode_free.argtypes = [ctypes.c_void_p]
_lib.f2v2f_encode_free.restype = None

_lib.f2v2f_decode_create.argtypes = []
_lib.f2v2f_decode_create.restype = ctypes.c_void_p

_lib.f2v2f_decode_create_with_params.argtypes = [
    ctypes.c_uint32,  # width
    ctypes.c_uint32,  # height
    ctypes.c_size_t,  # chunk_size
]
_lib.f2v2f_decode_create_with_params.restype = ctypes.c_void_p

_lib.f2v2f_decode_file.argtypes = [
    ctypes.c_void_p,  # handle
    ctypes.c_char_p,  # input_path
    ctypes.c_char_p,  # output_path
    ctypes.c_void_p,  # progress_callback
]
_lib.f2v2f_decode_file.restype = ctypes.c_int

_lib.f2v2f_decode_free.argtypes = [ctypes.c_void_p]
_lib.f2v2f_decode_free.restype = None

_lib.f2v2f_version.argtypes = []
_lib.f2v2f_version.restype = ctypes.c_char_p

_lib.f2v2f_get_last_error.argtypes = []
_lib.f2v2f_get_last_error.restype = ctypes.c_void_p  # Return raw pointer, not c_char_p

_lib.f2v2f_free_string.argtypes = [ctypes.c_void_p]  # Accept raw pointer
_lib.f2v2f_free_string.restype = None

# Initialize library
_lib.f2v2f_init()


class F2V2FError(Exception):
    """Base exception for f2v2f errors"""
    pass


class InvalidInputError(F2V2FError):
    """Raised when input is invalid"""
    pass


class EncodingError(F2V2FError):
    """Raised during encoding"""
    pass


class DecodingError(F2V2FError):
    """Raised during decoding"""
    pass


class ConfigError(F2V2FError):
    """Raised for configuration errors"""
    pass


def _check_error(error_code: int) -> None:
    """Convert error code to exception"""
    if error_code == ErrorCode.SUCCESS:
        return
    
    # Get the detailed error message from the library
    error_msg_ptr = _lib.f2v2f_get_last_error()
    error_msg = None
    if error_msg_ptr:
        try:
            # Cast raw pointer to c_char_p and decode
            error_msg = ctypes.cast(error_msg_ptr, ctypes.c_char_p).value.decode('utf-8')
        except:
            error_msg = None
        finally:
            # Free the string allocated by Rust
            _lib.f2v2f_free_string(error_msg_ptr)
    
    error_map = {
        ErrorCode.INVALID_INPUT: (InvalidInputError, error_msg or "Invalid input"),
        ErrorCode.IO_ERROR: (F2V2FError, error_msg or "I/O error"),
        ErrorCode.ENCODING_ERROR: (EncodingError, error_msg or "Encoding failed"),
        ErrorCode.DECODING_ERROR: (DecodingError, error_msg or "Decoding failed"),
        ErrorCode.CONFIG_ERROR: (ConfigError, error_msg or "Configuration error"),
        ErrorCode.INVALID_HANDLE: (F2V2FError, error_msg or "Invalid handle"),
        ErrorCode.UNKNOWN: (F2V2FError, error_msg or "Unknown error"),
    }
    
    exc_type, msg = error_map.get(error_code, (F2V2FError, error_msg or f"Unknown error code: {error_code}"))
    raise exc_type(msg)


class Encoder:
    """
    High-level encoder interface
    
    Example:
        encoder = Encoder(width=1920, height=1080, fps=30)
        encoder.encode("input.pdf", "output.mp4")
    """
    
    def __init__(self, width: int = 1920, height: int = 1080, fps: int = 30, chunk_size: int = 4096):
        """
        Create an encoder with the specified parameters
        
        Args:
            width: Video width in pixels
            height: Video height in pixels
            fps: Frames per second
            chunk_size: Size of file chunks in bytes
        """
        self.width = width
        self.height = height
        self.fps = fps
        self.chunk_size = chunk_size
        self._handle = None
    
    def _create_handle(self) -> None:
        """Create the encoder handle"""
        if self._handle is None:
            self._handle = _lib.f2v2f_encode_create(
                ctypes.c_uint32(self.width),
                ctypes.c_uint32(self.height),
                ctypes.c_uint32(self.fps),
                ctypes.c_size_t(self.chunk_size),
            )
            if self._handle is None:
                raise ConfigError("Failed to create encoder")
    
    def encode(self, input_path: str, output_path: str, progress_callback: Optional[Callable] = None) -> None:
        """
        Encode a file to video
        
        Args:
            input_path: Path to input file
            output_path: Path to output video file
            progress_callback: Optional callback function(bytes_total, frames_total, status_message)
        
        Raises:
            EncodingError: If encoding fails
            InvalidInputError: If input file is invalid
        """
        self._create_handle()
        
        # Create progress callback wrapper if provided
        callback_ptr = None
        if progress_callback:
            @ctypes.CFUNCTYPE(None, ctypes.c_uint64, ctypes.c_uint64, ctypes.c_char_p)
            def callback_wrapper(total_bytes, total_frames, msg):
                msg_str = msg.decode('utf-8') if msg else ""
                progress_callback(total_bytes, total_frames, msg_str)
            
            callback_ptr = ctypes.cast(callback_wrapper, ctypes.c_void_p)
        
        # Encode file
        error_code = _lib.f2v2f_encode_file(
            self._handle,
            input_path.encode('utf-8'),
            output_path.encode('utf-8'),
            callback_ptr,
        )
        
        _check_error(error_code)
    
    def __del__(self):
        """Clean up encoder handle"""
        if self._handle is not None:
            _lib.f2v2f_encode_free(self._handle)


class Decoder:
    """
    High-level decoder interface
    
    Example:
        decoder = Decoder()
        decoder.decode("output.mp4", "recovered.pdf")
    """
    
    def __init__(self, width: int = 1920, height: int = 1080, chunk_size: int = 4096):
        """
        Create a decoder
        
        Args:
            width: Expected video width
            height: Expected video height
            chunk_size: Data chunk size used during encoding
        """
        self.width = width
        self.height = height
        self.chunk_size = chunk_size
        self._handle = None
    
    def _create_handle(self) -> None:
        """Create the decoder handle"""
        if self._handle is None:
            self._handle = _lib.f2v2f_decode_create_with_params(
                ctypes.c_uint32(self.width),
                ctypes.c_uint32(self.height),
                ctypes.c_size_t(self.chunk_size),
            )
            if self._handle is None:
                raise ConfigError("Failed to create decoder")
    
    def decode(self, input_path: str, output_path: str, progress_callback: Optional[Callable] = None) -> None:
        """
        Decode a video back to a file
        
        Args:
            input_path: Path to input video file
            output_path: Path to output file
            progress_callback: Optional callback function(bytes_total, frames_extracted, status_message)
        
        Raises:
            DecodingError: If decoding fails
            InvalidInputError: If input video is invalid
        """
        self._create_handle()
        
        # Create progress callback wrapper if provided
        callback_ptr = None
        if progress_callback:
            @ctypes.CFUNCTYPE(None, ctypes.c_uint64, ctypes.c_uint64, ctypes.c_char_p)
            def callback_wrapper(total_bytes, total_frames, msg):
                msg_str = msg.decode('utf-8') if msg else ""
                progress_callback(total_bytes, total_frames, msg_str)
            
            callback_ptr = ctypes.cast(callback_wrapper, ctypes.c_void_p)
        
        # Decode file
        error_code = _lib.f2v2f_decode_file(
            self._handle,
            input_path.encode('utf-8'),
            output_path.encode('utf-8'),
            callback_ptr,
        )
        
        _check_error(error_code)
    
    def __del__(self):
        """Clean up decoder handle"""
        if self._handle is not None:
            _lib.f2v2f_decode_free(self._handle)


def version() -> str:
    """Get library version"""
    version_ptr = _lib.f2v2f_version()
    if version_ptr:
        return ctypes.c_char_p(version_ptr).value.decode('utf-8')
    return "unknown"
