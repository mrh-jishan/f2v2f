package main

import (
	"crypto/sha256"
	"fmt"
	"io"
	"log"
	"os"

	"github.com/mrh-jishan/f2v2f/backend/f2v2f"
)

func main() {
	// Initialize f2v2f
	f2v2f.Init()

	fmt.Println("🧪 Complete Encode→Decode Roundtrip Test")
	fmt.Println("==========================================")
	
	// Test files
	originalFile := "uploads/Screenshot 2026-02-15 at 09.08.16.png"
	encodedVideo := "outputs/test_roundtrip.mp4"
	decodedFile := "outputs/test_roundtrip_decoded.png"

	// Step 1: Get original file checksum
	fmt.Println("\n📋 Step 1: Calculate original file checksum")
	originalChecksum, originalSize, err := fileInfo(originalFile)
	if err != nil {
		log.Fatalf("Failed to read original: %v", err)
	}
	fmt.Printf("  Original: %s (%d bytes)\n", originalChecksum, originalSize)
	fmt.Printf("  Checksum: %s\n", originalChecksum)

	// Step 2: Encode
	fmt.Println("\n🎨 Step 2: Encode file to video")
	encoder, err := f2v2f.NewEncoder(1920, 1080, 30, 4096, true, 11)
	if err != nil {
		log.Fatalf("Failed to create encoder: %v", err)
	}
	defer encoder.Close()

	result, err := encoder.Encode(originalFile, encodedVideo)
	if err != nil {
		log.Fatalf("Encode failed: %v", err)
	}
	fmt.Printf("  ✅ Encoded to: %s\n", encodedVideo)
	fmt.Printf("  Encoded size: %d bytes\n", result.EncodedSize)
	fmt.Printf("  Chunk size: %d bytes\n", result.ChunkSize)

	videoStat, _ := os.Stat(encodedVideo)
	fmt.Printf("  Video file size: %d bytes\n", videoStat.Size())

	// Step 3: Decode
	fmt.Println("\n🎬 Step 3: Decode video back to file")
	decoder, err := f2v2f.NewDecoder(1920, 1080, result.ChunkSize, true, result.EncodedSize)
	if err != nil {
		log.Fatalf("Failed to create decoder: %v", err)
	}
	defer decoder.Close()

	err = decoder.Decode(encodedVideo, decodedFile)
	if err != nil {
		log.Fatalf("Decode failed: %v", err)
	}
	fmt.Printf("  ✅ Decoded to: %s\n", decodedFile)

	// Step 4: Verify
	fmt.Println("\n🔍 Step 4: Verify reconstruction")
	decodedChecksum, decodedSize, err := fileInfo(decodedFile)
	if err != nil {
		log.Fatalf("Failed to read decoded: %v", err)
	}
	fmt.Printf("  Decoded: %s (%d bytes)\n", decodedFile, decodedSize)
	fmt.Printf("  Checksum: %s\n", decodedChecksum)

	fmt.Println("\n==================================================")
	fmt.Println("FINAL VERIFICATION:")
	fmt.Println("==================================================")
	
	if originalChecksum == decodedChecksum {
		fmt.Println("✅ SUCCESS! Checksums match - PERFECT reconstruction!")
		if originalSize == decodedSize {
			fmt.Println("✅ File sizes match!")
		}
		fmt.Println("\n🎊 The encode→decode roundtrip is LOSSLESS!")
	} else {
		fmt.Println("❌ FAILED! Checksums do not match")
		fmt.Printf("  Original:  %s\n", originalChecksum)
		fmt.Printf("  Decoded:   %s\n", decodedChecksum)
		os.Exit(1)
	}
}

func fileInfo(path string) (checksum string, size int64, err error) {
	file, err := os.Open(path)
	if err != nil {
		return "", 0, err
	}
	defer file.Close()

	stat, err := file.Stat()
	if err != nil {
		return "", 0, err
	}

	hash := sha256.New()
	if _, err := io.Copy(hash, file); err != nil {
		return "", 0, err
	}

	return fmt.Sprintf("%x", hash.Sum(nil)), stat.Size(), nil
}
