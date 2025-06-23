#!/bin/bash

set -e

echo "Building all images with Docker Compose..."
docker compose build

echo "All images built successfully!"
