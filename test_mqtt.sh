#!/bin/bash

# Start the server in the background
echo "=== Starting MQTT server ==="
cd server
cargo run &
SERVER_PID=$!
cd ..

# Wait for server to initialize
echo "Waiting 3 seconds for server to start..."
sleep 3

# Run the client
echo ""
echo "=== Running client ==="
cd client
cargo run
cd ..

# Wait a bit to see server output after client disconnects
echo ""
echo "=== Waiting to see server response ==="
sleep 2

# Kill the server
echo ""
echo "=== Stopping server ==="
kill $SERVER_PID 2>/dev/null

echo "Done!"
