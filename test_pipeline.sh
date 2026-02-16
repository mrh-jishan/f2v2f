#!/bin/bash

# Configuration
API_URL="http://localhost:5000"
TEST_FILE="test_data.bin"
ENCODED_VIDEO="encoded_video.mp4"
RESTORED_FILE="restored_data.bin"
CHUNK_SIZE=256 # High redundancy (8100 pixels per byte)

# Cleanup
cleanup() {
    rm -f "$TEST_FILE" "$ENCODED_VIDEO" "$RESTORED_FILE"
}

# 1. Create test file (random data, 10KB - keep it small for high redundancy test)
echo "1. Creating test file..."
dd if=/dev/urandom of="$TEST_FILE" bs=1024 count=10 2>/dev/null
ORIGINAL_HASH=$(shasum -a 256 "$TEST_FILE" | awk '{print $1}')
echo "Original SHA256: $ORIGINAL_HASH"

# 2. Start Encoding
echo "2. Starting encoding job..."
RESPONSE=$(curl -s -X POST -F "file=@$TEST_FILE" -F "use_compression=true" -F "chunk_size=$CHUNK_SIZE" "$API_URL/api/encode")
JOB_ID=$(echo $RESPONSE | jq -r '.job_id')

if [ "$JOB_ID" == "null" ] || [ -z "$JOB_ID" ]; then
    echo "Failed to start encoding job: $RESPONSE"
    exit 1
fi
echo "Job ID: $JOB_ID"

# 3. Wait for Encoding to complete
echo "3. Waiting for encoding to complete..."
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
    sleep 1
done

# 4. Download Encoded Video
echo "4. Downloading encoded video..."
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

if [ "$DECODE_JOB_ID" == "null" ] || [ -z "$DECODE_JOB_ID" ]; then
    echo "Failed to start decoding job: $RESPONSE"
    exit 1
fi
echo "Decode Job ID: $DECODE_JOB_ID"

# 6. Wait for Decoding to complete
echo "6. Waiting for decoding to complete..."
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
    sleep 1
done

# 7. Download Restored File
echo "7. Downloading restored file..."
curl -s -o "$RESTORED_FILE" "$API_URL$DECODE_RESULT_URL"

# 8. Verify Results
echo "8. Verifying results..."
RESTORED_HASH=$(shasum -a 256 "$RESTORED_FILE" | awk '{print $1}')
echo "Restored SHA256: $RESTORED_HASH"

if [ "$ORIGINAL_HASH" == "$RESTORED_HASH" ]; then
    echo -e "\033[0;32mSUCCESS: File signature verified! byte-perfect reconstruction.\033[0m"
else
    echo -e "\033[0;31mFAILURE: File signature mismatch!\033[0m"
    echo "Original size: $(stat -f%z "$TEST_FILE")"
    echo "Restored size: $(stat -f%z "$RESTORED_FILE")"
    cmp -l "$TEST_FILE" "$RESTORED_FILE" | head -n 10
    exit 1
fi

cleanup
