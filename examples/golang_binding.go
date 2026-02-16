package main

import (
	"fmt"
	"log"
	"os"

	"github.com/mrh-jishan/f2v2f/backend/f2v2f"
)

func main() {
	// 1. Initialize f2v2f
	f2v2f.Init()
	fmt.Printf("f2v2f version: %s\n", f2v2f.Version())

	// 2. Setup paths
	inputFile := "example_input.txt"
	outputVideo := "example_output.mp4"
	recoveredFile := "example_recovered.txt"

	// Create test file
	err := os.WriteFile(inputFile, []byte("Hello, World! This is a Golang binding test for f2v2f."), 0644)
	if err != nil {
		log.Fatalf("Failed to create test file: %v", err)
	}
	defer os.Remove(inputFile)
	defer os.Remove(outputVideo)
	defer os.Remove(recoveredFile)

	// 3. Encode
	fmt.Println("📹 Encoding file to video...")
	encoder, err := f2v2f.NewEncoder(1920, 1080, 30, 4096, true, 3)
	if err != nil {
		log.Fatalf("Failed to create encoder: %v", err)
	}
	defer encoder.Close()

	encodedSize, err := encoder.Encode(inputFile, outputVideo)
	if err != nil {
		log.Fatalf("Encoding failed: %v", err)
	}
	fmt.Printf("✅ Encoding complete! Encoded size: %d bytes\n", encodedSize)

	// 4. Decode
	fmt.Println("🎬 Decoding video back to file...")
	decoder, err := f2v2f.NewDecoder(1920, 1080, 4096, true, encodedSize)
	if err != nil {
		log.Fatalf("Failed to create decoder: %v", err)
	}
	defer decoder.Close()

	err = decoder.Decode(outputVideo, recoveredFile)
	if err != nil {
		log.Fatalf("Decoding failed: %v", err)
	}
	fmt.Println("✅ Decoding complete!")

	// 5. Verify
	original, _ := os.ReadFile(inputFile)
	recovered, _ := os.ReadFile(recoveredFile)

	if string(original) == string(recovered) {
		fmt.Println("🎉 Success! Original and recovered files match perfectly.")
	} else {
		fmt.Println("❌ Error! Files do not match.")
		os.Exit(1)
	}
}
