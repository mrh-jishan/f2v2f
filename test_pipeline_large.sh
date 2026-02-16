#!/bin/bash

# Configuration
API_URL="http://localhost:5000"
TEST_FILE="test_large.bin"
ENCODED_VIDEO="encoded_video.mp4"
RESTORED_FILE="restored_large.bin"
CHUNK_SIZE=4096

# Cleanup
cleanup() {
    rm -f "$TEST_FILE" "$ENCODED_VIDEO" "$RESTORED_FILE"
}

# 1. Create test file (1MB of random data)
echo "1. Creating 1MB test file..."
dd if=/dev/urandom of="$TEST_FILE" bs=1024 count=1024 2>/dev/null
ORIGINAL_HASH=$(shasum -a 256 "$TEST_FILE" | awk '{print $1}')
echo "Original SHA256: $ORIGINAL_HASH"

# 2. Start Encoding
echo "2. Starting encoding job..."
RESPONSE=$(curl -s -X POST -F "file=@$TEST_FILE" -F "use_compression=true" -F "chunk_size=$CHUNK_SIZE" "$API_URL/api/encode")
JOB_ID=$(echo $RESPONSE | jq -r '.job_id')
echo "Job ID: $JOB_ID"

# 3. Wait for Encoding
echo "3. Waiting for encoding..."
while true; do
    STATUS_RES=$(curl -s "$API_URL/api/status/$JOB_ID")
    STATUS=$(echo $STATUS_RES | jq -r '.status')
    if [ "$STATUS" == "completed" ]; then
        echo -e "\nEncoding completed!"
        RESULT_URL=$(echo $STATUS_RES | jq -r '.result_url')
        ENCODED_SIZE=$(echo $STATUS_RES | jq -r '.encoded_data_size')
        break
    elif [ "$STATUS" == "failed" ]; then
        echo -e "\nEncoding failed: $(echo $STATUS_RES | jq -r '.error')"
        exit 1
    fi
    sleep 2
done

# 4. Download
echo "4. Downloading video..."
curl -s -o "$ENCODED_VIDEO" "$API_URL$RESULT_URL"

# 5. Start Decoding
echo "5. Starting decoding job..."
RESPONSE=$(curl -s -X POST \
  -F "file=@$ENCODED_VIDEO" \
  -F "use_compression=true" \
  -F "chunk_size=$CHUNK_SIZE" \
  -F "encoded_size=$ENCODED_SIZE" \
  "$API_URL/api/decode")
DECODE_JOB_ID=$(echo $RESPONSE | jq -r '.job_id')
echo "Decode Job ID: $DECODE_JOB_ID"

# 6. Wait for Decoding
echo "6. Waiting for decoding..."
while true; do
    STATUS_RES=$(curl -s "$API_URL/api/status/$DECODE_JOB_ID")
    STATUS=$(echo $STATUS_RES | jq -r '.status')
    if [ "$STATUS" == "completed" ]; then
        echo -e "\nDecoding completed!"
        DECODE_RESULT_URL=$(echo $STATUS_RES | jq -r '.result_url')
        break
    elif [ "$STATUS" == "failed" ]; then
        echo -e "\nDecoding failed: $(echo $STATUS_RES | jq -r '.error')"
        exit 1
    fi
    sleep 2
done

# 7. Download Restore
echo "7. Downloading restored file..."
curl -s -o "$RESTORED_FILE" "$API_URL$DECODE_RESULT_URL"

# 8. Verify
echo "8. Verifying results..."
RESTORED_HASH=$(shasum -a 256 "$RESTORED_FILE" | awk '{print $1}')
echo "Restored SHA256: $RESTORED_HASH"

if [ "$ORIGINAL_HASH" == "$RESTORED_HASH" ]; then
    echo -e "\033[0;32mSUCCESS: 1MB file signature verified! byte-perfect reconstruction.\033[0m"
else
    echo -e "\033[0;31mFAILURE: File signature mismatch!\033[0m"
    cmp -l "$TEST_FILE" "$RESTORED_FILE" | head -n 10
    exit 1
fi

cleanup
