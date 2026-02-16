"""
Flask backend for f2v2f web application

This provides REST API endpoints for encoding/decoding operations
with job queue support for long-running tasks.
"""

from flask import Flask, request, jsonify, send_file
from flask_cors import CORS
from werkzeug.utils import secure_filename
from pathlib import Path
import os
import uuid
import threading
from typing import Dict, Optional, List
from dataclasses import dataclass, asdict
from enum import Enum
import logging
import json
from datetime import datetime
import hashlib
import sqlite3

# Import f2v2f bindings
from f2v2f import Encoder, Decoder, F2V2FError

# Configuration
UPLOAD_FOLDER = Path(__file__).parent / "uploads"
OUTPUT_FOLDER = Path(__file__).parent / "outputs"
DB_PATH = Path(__file__).parent / "f2v2f.db"
ALLOWED_EXTENSIONS = {"*"}  # Allow all file extensions
MAX_FILE_SIZE = 5 * 1024 * 1024 * 1024  # 5GB

# Create folders
UPLOAD_FOLDER.mkdir(exist_ok=True)
OUTPUT_FOLDER.mkdir(exist_ok=True)

app = Flask(__name__)
app.config["UPLOAD_FOLDER"] = str(UPLOAD_FOLDER)
app.config["OUTPUT_FOLDER"] = str(OUTPUT_FOLDER)
app.config["MAX_CONTENT_LENGTH"] = MAX_FILE_SIZE

# Enable CORS for frontend communication
CORS(app, resources={
    r"/api/*": {
        "origins": "*",
        "methods": ["GET", "POST", "DELETE", "OPTIONS"],
        "allow_headers": ["Content-Type"]
    }
})

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)


class JobStatus(Enum):
    """Status of a job"""
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"


@dataclass
class Job:
    """Represents an encode/decode job"""
    job_id: str
    operation: str  # "encode" or "decode"
    input_file: str
    output_file: str
    status: JobStatus
    progress: int = 0
    error_message: Optional[str] = None
    result_url: Optional[str] = None
    original_filename: Optional[str] = None


@dataclass
class FileRecord:
    """Record of an encoded/decoded file"""
    id: str
    name: str
    type: str  # "original" or "encoded"
    size: int
    created_at: str
    video_url: Optional[str] = None
    original_file: Optional[str] = None
    checksum: Optional[str] = None


# In-memory job store (use database in production)
jobs: Dict[str, Job] = {}


def get_db():
    """Get database connection"""
    conn = sqlite3.connect(str(DB_PATH))
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    """Initialize SQLite database"""
    conn = get_db()
    cursor = conn.cursor()
    
    # Create files table
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS files (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            type TEXT NOT NULL,
            size INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            video_url TEXT,
            original_file TEXT,
            checksum TEXT,
            chunk_size INTEGER
        )
    ''')
    
    # Create index on created_at for faster queries
    cursor.execute('''
        CREATE INDEX IF NOT EXISTS idx_files_created_at 
        ON files(created_at DESC)
    ''')
    
    conn.commit()
    conn.close()
    logger.info("Database initialized")


def get_all_files() -> List[FileRecord]:
    """Get all files from database"""
    conn = get_db()
    cursor = conn.cursor()
    cursor.execute(
        'SELECT * FROM files ORDER BY created_at DESC'
    )
    rows = cursor.fetchall()
    conn.close()
    
    return [
        FileRecord(
            id=row['id'],
            name=row['name'],
            type=row['type'],
            size=row['size'],
            created_at=row['created_at'],
            video_url=row['video_url'],
            original_file=row['original_file'],
            checksum=row['checksum'],
            chunk_size=row['chunk_size']
        )
        for row in rows
    ]


def add_file_record(filename: str, file_type: str, size: int, video_url: Optional[str] = None, 
                   original_file: Optional[str] = None, checksum: Optional[str] = None,
                   chunk_size: Optional[int] = None):
    """Add a file to the database"""
    conn = get_db()
    cursor = conn.cursor()
    
    file_id = str(uuid.uuid4())
    created_at = datetime.now().isoformat()
    
    cursor.execute('''
        INSERT INTO files (id, name, type, size, created_at, video_url, original_file, checksum, chunk_size)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    ''', (file_id, filename, file_type, size, created_at, video_url, original_file, checksum, chunk_size))
    conn.commit()
    conn.close()
    
    logger.info(f"Added file record: {filename}")
    return FileRecord(
        id=file_id,
        name=filename,
        type=file_type,
        size=size,
        created_at=created_at,
        video_url=video_url,
        record.size,
        record.created_at,
        record.video_url,
        record.original_file,
        record.checksum
    ))
    conn.commit()
    conn.close()
    
    logger.info(f"Added file record: {filename}")
    return record


def calculate_checksum(filepath: Path) -> str:
    """Calculate SHA256 checksum of file"""
    sha256_hash = hashlib.sha256()
    with open(filepath, "rb") as f:
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
    return sha256_hash.hexdigest()


def allowed_file(filename: str) -> bool:
    """Check if file extension is allowed"""
    return True  # Allow all files


@app.route("/health", methods=["GET"])
def health_check():
    """Health check endpoint"""
    return jsonify({"status": "healthy"}), 200


@app.route("/api/version", methods=["GET"])
def get_version():
    """Get f2v2f library version"""
    from f2v2f import version
    return jsonify({"version": version()}), 200


@app.route("/api/encode", methods=["POST"])
def encode_file():
    """
    Encode a file to video
    
    Expected POST data:
    - file: (file) Input file
    - width: (int) Video width (default: 1920)
    - height: (int) Video height (default: 1080)
    - fps: (int) Frames per second (default: 30)
    """
    try:
        # Check if file is in request
        if "file" not in request.files:
            return jsonify({"error": "No file provided"}), 400
        
        file = request.files["file"]
        if file.filename == "":
            return jsonify({"error": "Empty filename"}), 400
        
        if not allowed_file(file.filename):
            return jsonify({"error": "File type not allowed"}), 400
        
        # Get parameters
        width = request.form.get("width", 1920, type=int)
        height = request.form.get("height", 1080, type=int)
        fps = request.form.get("fps", 30, type=int)
        chunk_size = request.form.get("chunk_size", 4096, type=int)  # 4KB default
        
        # Save uploaded file
        filename = secure_filename(file.filename)
        unique_name = f"{uuid.uuid4()}_{filename}"
        input_path = Path(app.config["UPLOAD_FOLDER"]) / unique_name
        file.save(input_path)
        
        # Determine output filename - Encode always results in MP4
        base_name = Path(filename).stem
        output_name = f"{uuid.uuid4()}_{base_name}.mp4"
        output_path = Path(app.config["OUTPUT_FOLDER"]) / output_name
        
        # Ensure output directory exists
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Create job
        job_id = str(uuid.uuid4())
        job = Job(
            job_id=job_id,
            operation="encode",
            input_file=str(input_path),
            output_file=str(output_path),
            status=JobStatus.PENDING,
        )
        jobs[job_id] = job
        
        # Start encoding in background thread
        def encode_task():
            job.status = JobStatus.RUNNING
            
            # Progress callback for this specific job
            def progress_callback(total_bytes: int, total_frames: int, message: str):
                logger.info(f"Job {job_id} progress: {total_bytes} bytes, {total_frames} frames - {message}")
                # Update job progress (rough estimate based on bytes processed)
                if hasattr(job, 'total_size') and job.total_size > 0:
                    job.progress = int((total_bytes / job.total_size) * 100)
                else:
                    job.progress = 50  # Default to 50% if can't calculate
            
            try:
                # Store total size for progress calculation
                job.total_size = input_path.stat().st_size
                
                logger.info(f"Starting encode for {input_path}")
                encoder = Encoder(width=width, height=height, fps=fps, chunk_size=chunk_size)
                encoder.encode(str(input_path), str(output_path), progress_callback)
                
                # Verify output file was created
                if not output_path.exists():
                    raise RuntimeError(f"Encoder did not create output file: {output_path}")
                
                job.status = JobStatus.COMPLETED
                job.progress = 100
                job.result_url = f"/api/download/{output_name}"
                
                # Add to file registry
                try:
                    output_size = output_path.stat().st_size
                    # Store original filename for future decoding
                    original_filename = file.filename
                    conn = get_db()
                    cursor = conn.cursor()
                    record_id = str(uuid.uuid4())
                    cursor.execute('''
                        INSERT INTO files (id, name, type, size, created_at, video_url, original_file, checksum)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    ''', (
                        record_id,
                        original_filename,
                        "encoded",
                        output_size,
                        datetime.now().isoformat(),
                        f"/api/download/{output_name}",
                        original_filename,  # Store original filename
                        None
                    ))
                    conn.commit()
                    conn.close()
                    logger.info(f"Added file record: {original_filename}")
                except Exception as e:
                    logger.warning(f"Could not add to registry: {e}")
                
                logger.info(f"Job {job_id} completed - output: {output_path} ({output_path.stat().st_size} bytes)")
            except Exception as e:
                job.status = JobStatus.FAILED
                job.error_message = str(e)
                logger.error(f"Job {job_id} failed: {e}", exc_info=True)
            finally:
                # Clean up input file
                try:
                    input_path.unlink()
                except:
                    pass
        
        thread = threading.Thread(target=encode_task, daemon=True)
        thread.start()
        
        return jsonify({
            "job_id": job_id,
            "status": job.status.value,
            "message": "Encoding started"
        }), 202
        
    except Exception as e:
        logger.error(f"Error in /api/encode: {e}")
        return jsonify({"error": str(e)}), 500


@app.route("/api/decode", methods=["POST"])
def decode_video():
    """
    Decode a video back to file
    
    Expected POST data:
    - file: (file) Input video file
    """
    try:
        if "file" not in request.files:
            return jsonify({"error": "No file provided"}), 400
        
        file = request.files["file"]
        if file.filename == "":
            return jsonify({"error": "Empty filename"}), 400
        
        # Save uploaded file
        filename = secure_filename(file.filename)
        unique_name = f"{uuid.uuid4()}_{filename}"
        input_path = Path(app.config["UPLOAD_FOLDER"]) / unique_name
        file.save(input_path)
        
        # Try to find original filename from database by looking up the video
        original_filename = None
        try:
            conn = get_db()
            cursor = conn.cursor()
            cursor.execute('SELECT original_file FROM files WHERE video_url = ? ORDER BY created_at DESC LIMIT 1', 
                          (f"/api/download/{filename}",))
            row = cursor.fetchone()
            if row and row['original_file']:
                original_filename = row['original_file']
            conn.close()
        except Exception as e:
            logger.warning(f"Could not look up original filename: {e}")
        
        # Determine output filename - preserve extension
        if original_filename:
            # Use original filename
            base_name = Path(original_filename).stem
            extension = Path(original_filename).suffix
            output_name = f"{uuid.uuid4()}_{base_name}{extension}"
        else:
            # Fallback to generic name
            output_name = f"{uuid.uuid4()}_decoded.bin"
        output_path = Path(app.config["OUTPUT_FOLDER"]) / output_name
        
        # Ensure output directory exists
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Create job
        job_id = str(uuid.uuid4())
        job = Job(
            job_id=job_id,
            operation="decode",
            input_file=str(input_path),
            output_file=str(output_path),
            status=JobStatus.PENDING,
        )
        jobs[job_id] = job
        
        # Start decoding in background thread
        def decode_task():
            job.status = JobStatus.RUNNING
            
            # Progress callback for this specific job
            def progress_callback(total_bytes: int, total_frames: int, message: str):
                logger.info(f"Job {job_id} progress: {total_bytes} bytes, {total_frames} frames - {message}")
                if hasattr(job, 'total_size') and job.total_size > 0:
                    job.progress = int((total_bytes / job.total_size) * 100)
                else:
                    job.progress = 50
            
            try:
                job.total_size = input_path.stat().st_size
                
                logger.info(f"Starting decode for {input_path}")
                decoder = Decoder()
                decoder.decode(str(input_path), str(output_path), progress_callback)
                
                # Verify output file was created
                if not output_path.exists():
                    raise RuntimeError(f"Decoder did not create output file: {output_path}")
                
                job.status = JobStatus.COMPLETED
                job.progress = 100
                job.result_url = f"/api/download/{output_name}"
                
                # Add to file registry
                try:
                    output_size = output_path.stat().st_size
                    checksum = calculate_checksum(output_path)
                    # Use the original filename if available
                    display_name = original_filename if original_filename else output_name
                    conn = get_db()
                    cursor = conn.cursor()
                    record_id = str(uuid.uuid4())
                    cursor.execute('''
                        INSERT INTO files (id, name, type, size, created_at, video_url, original_file, checksum)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    ''', (
                        record_id,
                        display_name,
                        "original",
                        output_size,
                        datetime.now().isoformat(),
                        f"/api/download/{output_name}",
                        original_filename,
                        checksum
                    ))
                    conn.commit()
                    conn.close()
                    logger.info(f"Added decoded file record: {display_name}")
                except Exception as e:
                    logger.warning(f"Could not add to registry: {e}")
                
                logger.info(f"Job {job_id} completed - output: {output_path} ({output_path.stat().st_size} bytes)")
            except Exception as e:
                job.status = JobStatus.FAILED
                job.error_message = str(e)
                logger.error(f"Job {job_id} failed: {e}", exc_info=True)
            finally:
                # Clean up input file
                try:
                    input_path.unlink()
                except:
                    pass
        
        thread = threading.Thread(target=decode_task, daemon=True)
        thread.start()
        
        return jsonify({
            "job_id": job_id,
            "status": job.status.value,
            "message": "Decoding started"
        }), 202
        
    except Exception as e:
        logger.error(f"Error in /api/decode: {e}")
        return jsonify({"error": str(e)}), 500


@app.route("/api/status/<job_id>", methods=["GET"])
def get_status(job_id: str):
    """Get status of a job"""
    try:
        if job_id not in jobs:
            logger.warning(f"Job not found: {job_id}")
            return jsonify({"error": "Job not found"}), 404
        
        job = jobs[job_id]
        response = {
            "job_id": job.job_id,
            "operation": job.operation,
            "status": job.status.value,
            "progress": job.progress,
        }
        
        if job.error_message:
            response["error"] = job.error_message
        if job.result_url:
            response["result_url"] = job.result_url
        
        return jsonify(response), 200
    except Exception as e:
        logger.error(f"Error getting job status {job_id}: {e}", exc_info=True)
        return jsonify({"error": "Internal server error"}), 500


@app.route("/api/download/<filename>", methods=["GET"])
def download_file(filename: str):
    """Download result file"""
    try:
        filepath = Path(app.config["OUTPUT_FOLDER"]) / secure_filename(filename)
        
        if not filepath.exists():
            return jsonify({"error": "File not found"}), 404
        
        return send_file(filepath, as_attachment=True, download_name=filename)
        
    except Exception as e:
        logger.error(f"Error downloading file: {e}")
        return jsonify({"error": str(e)}), 500


@app.route("/api/files", methods=["GET"])
def get_file_records():
    """Get all file records (history)"""
    try:
        files = get_all_files()
        records = [asdict(record) for record in files]
        return jsonify(records), 200
    except Exception as e:
        logger.error(f"Error fetching file records: {e}")
        return jsonify({"error": str(e)}), 500


@app.route("/api/files/<file_id>", methods=["DELETE"])
def delete_file_record(file_id: str):
    """Delete a file record and its associated files"""
    try:
        with get_db() as conn:
            cursor = conn.cursor()
            
            # Get file record
            cursor.execute('SELECT * FROM files WHERE id = ?', (file_id,))
            row = cursor.fetchone()
            
            if not row:
                return jsonify({"error": "File not found"}), 404
            
            # Extract video_url to get filename
            video_url = row[5]  # video_url is at index 5
            if video_url:
                filename = video_url.split('/')[-1]
                filepath = Path(app.config["OUTPUT_FOLDER"]) / filename
                if filepath.exists():
                    filepath.unlink()
                    logger.info(f"Deleted file: {filepath}")
            
            # Delete original file if it exists
            original_file = row[6]  # original_file is at index 6
            if original_file:
                orig_path = Path(app.config["UPLOAD_FOLDER"]) / original_file
                if orig_path.exists():
                    orig_path.unlink()
                    logger.info(f"Deleted original file: {orig_path}")
            
            # Delete database record
            cursor.execute('DELETE FROM files WHERE id = ?', (file_id,))
            conn.commit()
            
            logger.info(f"Deleted file record: {file_id}")
            return jsonify({"message": "File deleted successfully"}), 200
            
    except Exception as e:
        logger.error(f"Error deleting file record {file_id}: {e}")
        return jsonify({"error": str(e)}), 500


@app.route("/api/cleanup", methods=["POST"])
def cleanup_old_files():
    """Clean up old output files (admin endpoint)"""
    import time
    try:
        current_time = time.time()
        deleted_count = 0
        expiration_time = 24 * 60 * 60  # 24 hours
        
        for filepath in Path(app.config["OUTPUT_FOLDER"]).glob("*"):
            if filepath.is_file():
                if current_time - filepath.stat().st_mtime > expiration_time:
                    filepath.unlink()
                    deleted_count += 1
        
        return jsonify({"deleted_files": deleted_count}), 200
    except Exception as e:
        logger.error(f"Error during cleanup: {e}")
        return jsonify({"error": str(e)}), 500


@app.errorhandler(413)
def too_large(e):
    """Handle file too large"""
    return jsonify({"error": "File too large. Maximum size is 5GB"}), 413


@app.errorhandler(500)
def internal_error(e):
    """Handle internal server error"""
    logger.error(f"Internal error: {e}")
    return jsonify({"error": "Internal server error"}), 500


# Initialize database when app starts
init_db()

if __name__ == "__main__":
    # Disable auto-reload in debug mode to prevent Tokio runtime issues
    app.run(debug=True, use_reloader=False, host="0.0.0.0", port=5001)
