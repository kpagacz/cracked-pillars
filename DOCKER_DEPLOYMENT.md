# Docker Deployment Guide

This guide explains how the Cracked Pillars application is deployed to production using Docker, AWS ECR, and EC2.

## Architecture

The application consists of two main services:

- **Frontend (chisel)**: Next.js application running on port 3000
- **Backend (hammer)**: Rust Axum API running on port 8001 with SQLite database

## Production Deployment Process

### 1. Image Building and Publishing

Images are automatically built and published via GitHub Actions pipeline:

- **Build Pipeline**: GitHub Actions automatically builds Docker images when code is pushed
- **ECR Registry**: Images are pushed to AWS ECR registry
- **Registry URLs**: See `push-images-to-ecr.sh` for the specific ECR repository URLs

### 2. Deployment Branch

A separate deployment branch contains the files needed for production deployment:

- `docker-compose.prod.yml` - Production Docker Compose configuration
- `hammer.db3` - SQLite database file
- Other deployment-specific files

### 3. EC2 Deployment

The application is deployed on an EC2 instance with the following setup:

#### Prerequisites
- EC2 instance with Docker and Docker Compose installed
- IAM role configured to pull images from ECR
- Access to the deployment branch

#### Deployment Steps

1. **Log into ECR on the EC2 instance:**
   ```bash
   aws ecr get-login-password --region <region> | docker login --username AWS --password-stdin <ecr-registry-url>
   ```

2. **Pull the latest images:**
   ```bash
   docker compose -f docker-compose.prod.yml pull
   ```

3. **Deploy the application:**
   ```bash
   docker-compose -f docker-compose.prod.yml up -d
   ```

### 4. Database Configuration

- **Database Mount**: The SQLite database (`hammer.db3`) is mounted directly onto the backend container
- **No Backup Strategy**: Currently, there is no automated database backup system in place
- **Data Persistence**: Database changes are persisted through the volume mount

## Development Setup

For local development, you can use the development Docker Compose configuration:

```bash
# Clone the repository
git clone <repository-url>
cd cracked-pillars

# Start development environment
docker compose up --build -d
```

## Service Details

### Frontend (chisel)
- **Port**: 3000
- **Technology**: Next.js 15 with TypeScript
- **Build**: Multi-stage Docker build with standalone output
- **Environment**: Production mode with telemetry disabled

### Backend (hammer)
- **Port**: 8001
- **Technology**: Rust with Axum framework
- **Database**: SQLite with direct file mount
- **Data**: Mounts JSON files for abilities and items

## Environment Variables

### Backend Environment Variables
- `RUST_LOG`: Logging level (default: `hammer=info,tower_http=info`)
- `HAMMER_*`: Configuration overrides (see config.rs)

### Frontend Environment Variables
- `NODE_ENV`: Set to `production`
- `NEXT_TELEMETRY_DISABLED`: Disabled for privacy

## Useful Commands

### View logs
```bash
# All services
docker compose -f docker-compose.prod.yml logs -f

# Specific service
docker compose -f docker-compose.prod.yml logs -f backend
docker compose -f docker-compose.prod.yml logs -f frontend
```

### Stop services
```bash
docker compose -f docker-compose.prod.yml down
```

### Restart services
```bash
docker compose -f docker-compose.prod.yml restart
```

### Access container shell
```bash
docker compose -f docker-compose.prod.yml exec backend sh
docker compose -f docker-compose.prod.yml exec frontend sh
```

## Troubleshooting

### ECR Authentication Issues
If you encounter authentication issues with ECR:

```bash
# Re-authenticate with ECR
aws ecr get-login-password --region <region> | docker login --username AWS --password-stdin <ecr-registry-url>
```

### Database Issues
If the database becomes corrupted:

```bash
# Stop services
docker compose -f docker-compose.prod.yml down

# Backup current database (if needed)
cp hammer.db3 hammer.db3.backup

# Restart services (will use backup or recreate if needed)
docker compose -f docker-compose.prod.yml up -d
```

### Image Pull Issues
If images fail to pull:

```bash
# Check ECR authentication
aws sts get-caller-identity

# Verify IAM role has ECR permissions
aws ecr describe-repositories
```

## Security Considerations

1. **ECR Access**: Ensure EC2 instance has proper IAM role for ECR access
2. **Database Security**: SQLite database is mounted directly - ensure file permissions are secure
3. **Network Security**: Configure security groups to restrict access to necessary ports only
4. **No Backup**: Currently no automated backup strategy - consider implementing one

## File Structure

```
cracked-pillars/
├── docker-compose.yml          # Development orchestration
├── docker-compose.prod.yml     # Production orchestration
├── build-images.sh            # Local image building script
├── push-images-to-ecr.sh      # ECR push script with registry URLs
├── chisel/
│   ├── Dockerfile             # Frontend container
│   └── .dockerignore
├── hammer/
│   ├── Dockerfile             # Backend container
│   ├── .dockerignore
│   ├── hammer.toml            # Configuration
│   └── hammer.db3             # SQLite database (in deployment branch)
└── DOCKER_DEPLOYMENT.md       # This file
```
