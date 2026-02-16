package main

import (
	"database/sql"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"time"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"github.com/google/uuid"
	_ "modernc.org/sqlite"

	"github.com/mrh-jishan/f2v2f/backend/f2v2f"
)

// Config
const (
	UploadDir   = "uploads"
	OutputDir   = "outputs"
	DBPath      = "f2v2f.db"
	MaxFileSize = 50 * 1024 * 1024 * 1024 // 50GB for Go server
)

type JobStatus string

const (
	StatusPending   JobStatus = "pending"
	StatusRunning   JobStatus = "running"
	StatusCompleted JobStatus = "completed"
	StatusFailed    JobStatus = "failed"
)

type Job struct {
	ID               string    `json:"job_id"`
	Operation        string    `json:"operation"`
	Status           JobStatus `json:"status"`
	Progress         int       `json:"progress"`
	ErrorMessage     string    `json:"error,omitempty"`
	ResultURL        string    `json:"result_url,omitempty"`
	OriginalFilename string    `json:"original_filename"`
	EncodedDataSize  uint64    `json:"encoded_data_size,omitempty"`
}

type FileRecord struct {
	ID               string `json:"id"`
	Name             string `json:"name"`
	Type             string `json:"type"`
	Size             int64  `json:"size"`
	CreatedAt        string `json:"created_at"`
	VideoURL         string `json:"video_url,omitempty"`
	OriginalFile     string `json:"original_file,omitempty"`
	Checksum         string `json:"checksum,omitempty"`
	ChunkSize        int    `json:"chunk_size,omitempty"`
	UnencodedSize    int64  `json:"unencoded_size,omitempty"`
	UseCompression   bool   `json:"use_compression"`
	CompressionLevel int    `json:"compression_level"`
	EncodedDataSize  int64  `json:"encoded_data_size,omitempty"`
}

var (
	jobs = make(map[string]*Job)
	db   *sql.DB
)

func init() {
	os.MkdirAll(UploadDir, 0755)
	os.MkdirAll(OutputDir, 0755)

	var err error
	db, err = sql.Open("sqlite", DBPath)
	if err != nil {
		log.Fatal(err)
	}

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS files (
			id TEXT PRIMARY KEY,
			name TEXT NOT NULL,
			type TEXT NOT NULL,
			size INTEGER NOT NULL,
			created_at TEXT NOT NULL,
			video_url TEXT,
			original_file TEXT,
			checksum TEXT,
			chunk_size INTEGER,
			unencoded_size INTEGER,
			use_compression INTEGER DEFAULT 0,
			compression_level INTEGER DEFAULT 3,
			encoded_data_size INTEGER
		)
	`)
	if err != nil {
		log.Fatal(err)
	}

	f2v2f.Init()
}

func main() {
	app := fiber.New(fiber.Config{
		BodyLimit: MaxFileSize,
	})

	app.Use(logger.New())
	app.Use(cors.New())

	app.Get("/health", func(c *fiber.Ctx) error {
		return c.JSON(fiber.Map{"status": "healthy", "engine": "golang"})
	})

	app.Get("/api/version", func(c *fiber.Ctx) error {
		return c.JSON(fiber.Map{"version": f2v2f.Version()})
	})

	app.Post("/api/encode", handleEncode)
	app.Post("/api/decode", handleDecode)
	app.Get("/api/status/:job_id", handleStatus)
	app.Get("/api/download/:filename", handleDownload)
	app.Get("/api/files", handleListFiles)

	log.Fatal(app.Listen(":5001"))
}

func handleEncode(c *fiber.Ctx) error {
	file, err := c.FormFile("file")
	if err != nil {
		return c.Status(400).JSON(fiber.Map{"error": "No file provided"})
	}

	width := c.FormValue("width", "1920")
	height := c.FormValue("height", "1080")
	fps := c.FormValue("fps", "30")
	chunkSize := c.FormValue("chunk_size", "4096")
	useCompression := c.FormValue("use_compression", "true") == "true"
	compressionLevel := c.FormValue("compression_level", "3")

	jobID := uuid.New().String()
	uniqueName := fmt.Sprintf("%s_%s", uuid.New().String(), file.Filename)
	inputPath := filepath.Join(UploadDir, uniqueName)

	if err := c.SaveFile(file, inputPath); err != nil {
		return c.Status(500).JSON(fiber.Map{"error": err.Error()})
	}

	outputName := fmt.Sprintf("%s_%s.mp4", uuid.New().String(), uuid.New().String())
	outputPath := filepath.Join(OutputDir, outputName)

	job := &Job{
		ID:               jobID,
		Operation:        "encode",
		Status:           StatusPending,
		OriginalFilename: file.Filename,
	}
	jobs[jobID] = job

	go func() {
		job.Status = StatusRunning

		// Parse params (ignoring errors for brevity, uses defaults)
		w := uint32(1920)
		fmt.Sscanf(width, "%d", &w)
		h := uint32(1080)
		fmt.Sscanf(height, "%d", &h)
		f := uint32(30)
		fmt.Sscanf(fps, "%d", &f)
		cs := 4096
		fmt.Sscanf(chunkSize, "%d", &cs)
		cl := 3
		fmt.Sscanf(compressionLevel, "%d", &cl)

		encoder, err := f2v2f.NewEncoder(w, h, f, cs, useCompression, cl)
		if err != nil {
			job.Status = StatusFailed
			job.ErrorMessage = err.Error()
			return
		}
		defer encoder.Close()

		encodedSize, err := encoder.Encode(inputPath, outputPath)
		if err != nil {
			job.Status = StatusFailed
			job.ErrorMessage = err.Error()
			return
		}

		job.Status = StatusCompleted
		job.Progress = 100
		job.ResultURL = fmt.Sprintf("/api/download/%s", outputName)
                job.EncodedDataSize = encodedSize
		// Record in DB
		stat, _ := os.Stat(outputPath)
		_, _ = db.Exec(`
			INSERT INTO files (id, name, type, size, created_at, video_url, original_file, chunk_size, unencoded_size, use_compression, compression_level, encoded_data_size)
			VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
		`, uuid.New().String(), outputName, "encoded", stat.Size(), time.Now().Format(time.RFC3339), job.ResultURL, file.Filename, cs, file.Size, useCompression, cl, encodedSize)

		os.Remove(inputPath)
	}()

	return c.Status(202).JSON(fiber.Map{
		"job_id":  jobID,
		"status":  job.Status,
		"message": "Encoding started",
	})
}

func handleDecode(c *fiber.Ctx) error {
	file, err := c.FormFile("file")
	if err != nil {
		return c.Status(400).JSON(fiber.Map{"error": "No file provided"})
	}

	jobID := uuid.New().String()
	uniqueName := fmt.Sprintf("%s_%s", uuid.New().String(), file.Filename)
	inputPath := filepath.Join(UploadDir, uniqueName)

	if err := c.SaveFile(file, inputPath); err != nil {
		return c.Status(500).JSON(fiber.Map{"error": err.Error()})
	}

	// Try to get params from form, fallback to DB
	chunkSizeStr := c.FormValue("chunk_size")
	useCompressionStr := c.FormValue("use_compression")
	encodedSizeStr := c.FormValue("encoded_size")

	var chunkSize int = 4096
	var useCompression bool = false
	var encodedDataSize uint64 = 0
	var originalFile string = "decoded.bin"

	// If any param is missing, try DB lookup as fallback
	if chunkSizeStr == "" || useCompressionStr == "" || encodedSizeStr == "" {
		_ = db.QueryRow("SELECT chunk_size, use_compression, encoded_data_size, original_file FROM files WHERE name = ? OR video_url LIKE ? ORDER BY created_at DESC LIMIT 1",
			file.Filename, "%"+file.Filename).Scan(&chunkSize, &useCompression, &encodedDataSize, &originalFile)
	}

	// Override with form values if provided
	if chunkSizeStr != "" {
		fmt.Sscanf(chunkSizeStr, "%d", &chunkSize)
	}
	if useCompressionStr != "" {
		useCompression = useCompressionStr == "true"
	}
	if encodedSizeStr != "" {
		fmt.Sscanf(encodedSizeStr, "%d", &encodedDataSize)
	}

	outputName := fmt.Sprintf("%s_%s", uuid.New().String(), originalFile)
	outputPath := filepath.Join(OutputDir, outputName)

	job := &Job{
		ID:               jobID,
		Operation:        "decode",
		Status:           StatusPending,
		OriginalFilename: file.Filename,
	}
	jobs[jobID] = job

	go func() {
		job.Status = StatusRunning

		decoder, err := f2v2f.NewDecoder(1920, 1080, chunkSize, useCompression, encodedDataSize)
		if err != nil {
			job.Status = StatusFailed
			job.ErrorMessage = err.Error()
			return
		}
		defer decoder.Close()

		if err := decoder.Decode(inputPath, outputPath); err != nil {
			job.Status = StatusFailed
			job.ErrorMessage = err.Error()
			return
		}

		job.Status = StatusCompleted
		job.Progress = 100
		job.ResultURL = fmt.Sprintf("/api/download/%s", outputName)

		stat, _ := os.Stat(outputPath)
		_, _ = db.Exec(`
			INSERT INTO files (id, name, type, size, created_at, video_url, original_file)
			VALUES (?, ?, ?, ?, ?, ?, ?)
		`, uuid.New().String(), outputName, "original", stat.Size(), time.Now().Format(time.RFC3339), job.ResultURL, originalFile)

		os.Remove(inputPath)
	}()

	return c.Status(202).JSON(fiber.Map{
		"job_id":  jobID,
		"status":  job.Status,
		"message": "Decoding started",
	})
}

func handleStatus(c *fiber.Ctx) error {
	jobID := c.Params("job_id")
	job, ok := jobs[jobID]
	if !ok {
		return c.Status(404).JSON(fiber.Map{"error": "Job not found"})
	}
	return c.JSON(job)
}

func handleDownload(c *fiber.Ctx) error {
	filename := c.Params("filename")
	path := filepath.Join(OutputDir, filename)
	if _, err := os.Stat(path); os.IsNotExist(err) {
		return c.Status(404).JSON(fiber.Map{"error": "File not found"})
	}
	return c.Download(path)
}

func handleListFiles(c *fiber.Ctx) error {
	rows, err := db.Query("SELECT * FROM files ORDER BY created_at DESC")
	if err != nil {
		return c.Status(500).JSON(fiber.Map{"error": err.Error()})
	}
	defer rows.Close()

	var results []FileRecord
	for rows.Next() {
		var r FileRecord
		var videoURL, originalFile, checksum sql.NullString
		var chunkSize, unencodedSize, useCompression, compressionLevel, encodedDataSize sql.NullInt64

		err = rows.Scan(&r.ID, &r.Name, &r.Type, &r.Size, &r.CreatedAt, &videoURL, &originalFile, &checksum, &chunkSize, &unencodedSize, &useCompression, &compressionLevel, &encodedDataSize)
		if err != nil {
			return c.Status(500).JSON(fiber.Map{"error": err.Error()})
		}
		r.VideoURL = videoURL.String
		r.OriginalFile = originalFile.String
		r.Checksum = checksum.String
		r.ChunkSize = int(chunkSize.Int64)
		r.UnencodedSize = unencodedSize.Int64
		r.UseCompression = useCompression.Int64 == 1
		r.CompressionLevel = int(compressionLevel.Int64)
		r.EncodedDataSize = encodedDataSize.Int64
		results = append(results, r)
	}

	return c.JSON(results)
}
