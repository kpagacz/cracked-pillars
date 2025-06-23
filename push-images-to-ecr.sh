#!/bin/bash

set -e

AWS_REGION="eu-central-1"
BACKEND_ECR_URI="565855251853.dkr.ecr.eu-central-1.amazonaws.com/cracked-pillars-backend"
FRONTEND_ECR_URI="565855251853.dkr.ecr.eu-central-1.amazonaws.com/cracked-pillars-frontend"
BACKEND_LOCAL_IMAGE="cracked-pillars-backend:latest"
FRONTEND_LOCAL_IMAGE="cracked-pillars-frontend:latest"
BACKEND_REMOTE_IMAGE="$BACKEND_ECR_URI:latest"
FRONTEND_REMOTE_IMAGE="$FRONTEND_ECR_URI:latest"

# Authenticate Docker to your ECR registry
aws ecr get-login-password --region $AWS_REGION | docker login --username AWS --password-stdin 565855251853.dkr.ecr.eu-central-1.amazonaws.com

echo "Pushing backend image..."
docker tag $BACKEND_LOCAL_IMAGE $BACKEND_REMOTE_IMAGE
docker push $BACKEND_REMOTE_IMAGE

echo "Pushing frontend image..."
docker tag $FRONTEND_LOCAL_IMAGE $FRONTEND_REMOTE_IMAGE
docker push $FRONTEND_REMOTE_IMAGE

echo "Both images pushed successfully!"
