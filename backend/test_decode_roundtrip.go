package main

import (
	"crypto/sha256"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"

	"github.com/mrh-jishan/f2v2f/backend/f2v2f"
)

func main() {
	// Initialize f2v2f
	f2v2f.Init()

	// Test parameters from database
	videoFile := "outputs/3c424f14-e506-4c5d-bbfb-685d2abc6d42_897bb69c-20ab-4de0-9fed-11f70f670684.mp4"
	originalFile := "uploads/Screenshot 2026-02-15 at 09.08.16.png"
	outputFile := "outputs/test_decoded_screenshot.png"

	// Parameters from database query
	chunkSize := 4096
	useCompression := true
	encodedDataSize := uint64(197670)

	fmt.Println("🧪 Testing Decode Roundtrip")
	fmt.Println("========================================")
	fmt.Printf("Video file: %s\n", videoFile)
	fmt.Printf("Original file: %s\n", originalFile)
	fmt.Printf("Parameters: chunk_size=%d, use_compression=%v, encoded_data_size=%d\n", 
		chunkSize, useCompression, encodedDataSize)
	fmt.Println()

	// Calculate checksum of original file
	fmt.Println("📋 Calculating checksum of original file...")
	originalChecksum, err := fileChecksum(originalFile)
	if err != nil {
		log.Fatalf("Failed to checksum original: %v", err)
	}
	fmt.Printf("Original checksum: %s\n\n", originalChecksum)

	// Decode the video
	fmt.Println("🎬 Decoding video to file...")
	decoder, err := f2v2f.NewDecoder(1920, 1080, chunkSize, useCompression, encodedDataSize)
	if err != nil {
		log.Fatalf("Failed to create decoder: %v", err)
	}
	defer decoder.Close()

	err = decoder.Decode(videoFile, outputFile)
	if err != nil {
		log.Fatalf("Decode failed: %v", err)
	}
	fmt.Printf("✅ Decoded to: %s\n\n", outputFile)

	// Calculate checksum of decoded file
	fmt.Println("📋 Calculating checksum of decoded file...")
	decodedChecksum, err := fileChecksum(outputFile)
	if err != nil {
		log.Fatalf("Failed to checksum decoded: %v", err)
	}
	fmt.Printf("Decoded checksum: %s\n\n", decodedChecksum)

	// Compare
	fmt.Println("🔍 Verification:")
	if originalChecksum == decodedChecksum {
		fmt.Println("✅ SUCCESS! Checksums match - file perfectly restored!")
		
		// Compare file sizes
		originalInfo, _ := os.Stat(originalFile)
		decodedInfo, _ := os.Stat(outputFile)
		fmt.Printf("Original size: %d bytes\n", originalInfo.Size())
		fmt.Printf("Decoded size:  %d bytes\n", decodedInfo.Size())
		
		if originalInfo.Size() == decodedInfo.Size() {
			fmt.Println("✅ File sizes match!")
		}
	} else {
		fmt.Println("❌ FAILED! Checksums do not match")
		fmt.Printf("Expected: %s\n", originalChecksum)
		fmt.Printf("Got:      %s\n", decodedChecksum)
		os.Exit(1)
	}
}

func fileChecksum(path string) (string, error) {
	file, err := os.Open(path)
	if err != nil {
		return "", err
	}
	defer file.Close()

	// Resolve any symlinks to get absolute path
	absPath, err := filepath.Abs(path)
	if err == nil {
		file, err = os.Open(absPath)
		if err != nil {
			return "", err
		}
		defer file.Close()
	}

	hash := sha256.New()
	if _, err := io.Copy(hash, file); err != nil {
		return "", err
	}

	return fmt.Sprintf("%x", hash.Sum(nil)), nil
}
