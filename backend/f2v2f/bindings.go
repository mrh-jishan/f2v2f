package f2v2f

/*
#cgo LDFLAGS: -L${SRCDIR}/../../lib/target/release -lf2v2f
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

int32_t f2v2f_init();
void* f2v2f_encode_create(uint32_t width, uint32_t height, uint32_t fps, size_t chunk_size, bool use_compression, int32_t compression_level);
int32_t f2v2f_encode_file(void* handle, const char* input_path, const char* output_path, uint64_t* encoded_size_out, size_t* chunk_size_out, void* progress_callback);
void f2v2f_encode_free(void* handle);
void* f2v2f_decode_create_with_params(uint32_t width, uint32_t height, size_t chunk_size, bool use_compression, uint64_t encoded_size);
int32_t f2v2f_decode_file(void* handle, const char* input_path, const char* output_path, void* progress_callback);
void f2v2f_decode_free(void* handle);
char* f2v2f_version();
char* f2v2f_get_last_error();
void f2v2f_free_string(char* s);
*/
import "C"
import (
	"errors"
	"unsafe"
)

func Init() int32 {
	return int32(C.f2v2f_init())
}

func Version() string {
	res := C.f2v2f_version()
	return C.GoString(res)
}

func getLastError() error {
	ptr := C.f2v2f_get_last_error()
	if ptr == nil {
		return errors.New("unknown error")
	}
	defer C.f2v2f_free_string(ptr)
	return errors.New(C.GoString(ptr))
}

type Encoder struct {
	handle unsafe.Pointer
}

type EncodeResult struct {
	EncodedSize uint64
	ChunkSize   int
}

func NewEncoder(width, height, fps uint32, chunkSize int, useCompression bool, compressionLevel int) (*Encoder, error) {
	handle := C.f2v2f_encode_create(
		C.uint32_t(width),
		C.uint32_t(height),
		C.uint32_t(fps),
		C.size_t(chunkSize),
		C.bool(useCompression),
		C.int32_t(compressionLevel),
	)
	if handle == nil {
		return nil, errors.New("failed to create encoder")
	}
	return &Encoder{handle: handle}, nil
}

func (e *Encoder) Encode(inputPath, outputPath string) (*EncodeResult, error) {
	cInput := C.CString(inputPath)
	defer C.free(unsafe.Pointer(cInput))
	cOutput := C.CString(outputPath)
	defer C.free(unsafe.Pointer(cOutput))

	var encodedSize C.uint64_t
	var actualChunkSize C.size_t
	res := C.f2v2f_encode_file(e.handle, cInput, cOutput, &encodedSize, &actualChunkSize, nil)
	if res != 0 {
		return nil, getLastError()
	}
	return &EncodeResult{
		EncodedSize: uint64(encodedSize),
		ChunkSize:   int(actualChunkSize),
	}, nil
}

func (e *Encoder) Close() {
	if e.handle != nil {
		C.f2v2f_encode_free(e.handle)
		e.handle = nil
	}
}

type Decoder struct {
	handle unsafe.Pointer
}

func NewDecoder(width, height uint32, chunkSize int, useCompression bool, encodedSize uint64) (*Decoder, error) {
	handle := C.f2v2f_decode_create_with_params(
		C.uint32_t(width),
		C.uint32_t(height),
		C.size_t(chunkSize),
		C.bool(useCompression),
		C.uint64_t(encodedSize),
	)
	if handle == nil {
		return nil, errors.New("failed to create decoder")
	}
	return &Decoder{handle: handle}, nil
}

func (d *Decoder) Decode(inputPath, outputPath string) error {
	cInput := C.CString(inputPath)
	defer C.free(unsafe.Pointer(cInput))
	cOutput := C.CString(outputPath)
	defer C.free(unsafe.Pointer(cOutput))

	res := C.f2v2f_decode_file(d.handle, cInput, cOutput, nil)
	if res != 0 {
		return getLastError()
	}
	return nil
}

func (d *Decoder) Close() {
	if d.handle != nil {
		C.f2v2f_decode_free(d.handle)
		d.handle = nil
	}
}
