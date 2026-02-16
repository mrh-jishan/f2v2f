package main

import (
	"fmt"
	"os"

	"github.com/mrh-jishan/f2v2f/backend/f2v2f"
)

func main() {
	if code := f2v2f.Init(); code != 0 {
		fmt.Printf("Init failed: %d\n", code)
		os.Exit(1)
	}

	encoder, err := f2v2f.NewEncoder(1920, 1080, 30, 4096, true, 11)
	if err != nil {
		fmt.Printf("Create encoder failed: %v\n", err)
		os.Exit(1)
	}
	defer encoder.Close()

	fmt.Println("Testing small file encoding...")
	encodedSize, err := encoder.Encode("test_small.txt", "test_output.mp4")
	if err != nil {
		fmt.Printf("Encoding failed: %v\n", err)
		os.Exit(1)
	}

	fmt.Printf("Success! Encoded size: %d bytes\n", encodedSize)
}
