#!/bin/bash

set -e

# Configuration
DOCKER_IMAGE="johnsonf/stacks-bot:latest"
CONTAINER_NAME="stacks-bot"

echo "🚀 Deploying stacks-bot to VPS..."

# Copy .env file to VPS
echo "📄 Copying .env file to VPS..."
scp .env myserver:~/.env

# SSH into VPS and execute deployment commands
ssh myserver bash << ENDSSH
set -e

export DOCKER_IMAGE="${DOCKER_IMAGE}"
export CONTAINER_NAME="${CONTAINER_NAME}"

echo "📦 Pulling latest Docker image..."
docker pull \${DOCKER_IMAGE}

echo "🛑 Stopping existing container (if running)..."
docker stop \${CONTAINER_NAME} 2>/dev/null || true
docker rm \${CONTAINER_NAME} 2>/dev/null || true

echo "🏃 Starting new container..."
docker run -d \
  --name \${CONTAINER_NAME} \
  --restart unless-stopped \
  --env-file ~/.env \
  \${DOCKER_IMAGE}

echo "✅ Deployment complete!"
docker ps | grep \${CONTAINER_NAME}

ENDSSH

echo "✨ Done! Container is running on VPS."
