# Docker Deployment Guide

This guide explains how to deploy the Cracked Pillars application using Docker Compose.

## Architecture

The application consists of two main services:

- **Frontend (chisel)**: Next.js application running on port 3000
- **Backend (hammer)**: Rust Axum API running on port 8001

## Prerequisites

- Docker
- Docker Compose

## Quick Start

1. **Clone and navigate to the project directory:**
   ```bash
   cd /path/to/cracked-pillars
   ```

2. **Build and start the services:**
   ```bash
   docker-compose up --build
   ```

3. **Access the application:**
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8001
   - Health check: http://localhost:8001/health

## Service Details

### Frontend (chisel)
- **Port**: 3000
- **Technology**: Next.js 15 with TypeScript
- **Build**: Multi-stage Docker build with standalone output
- **Environment**: Production mode with telemetry disabled

### Backend (hammer)
- **Port**: 8001
- **Technology**: Rust with Axum framework
- **Database**: SQLite with persistent volume storage
- **Data**: Mounts JSON files for abilities and items

## Data Persistence

The SQLite database is stored in a Docker volume (`hammer_data`) to ensure data persistence across container restarts.

## Environment Variables

### Backend Environment Variables
- `RUST_LOG`: Logging level (default: `hammer=info,tower_http=info`)
- `HAMMER_*`: Configuration overrides (see config.rs)

### Frontend Environment Variables
- `NODE_ENV`: Set to `production`
- `NEXT_TELEMETRY_DISABLED`: Disabled for privacy

## Useful Commands

### Start services in background
```bash
docker-compose up -d --build
```

### View logs
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend
docker-compose logs -f frontend
```

### Stop services
```bash
docker-compose down
```

### Stop and remove volumes
```bash
docker-compose down -v
```

### Rebuild a specific service
```bash
docker-compose build backend
docker-compose build frontend
```

### Access container shell
```bash
docker-compose exec backend sh
docker-compose exec frontend sh
```

## Development

For development, you can override the docker-compose configuration:

```bash
# Create a development override file
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up
```

## Troubleshooting

### Port conflicts
If ports 3000 or 8001 are already in use, modify the port mappings in `docker-compose.yml`:

```yaml
ports:
  - "3001:3000"  # Map host port 3001 to container port 3000
```

### Database issues
If the database becomes corrupted or you need to reset it:

```bash
# Stop services
docker-compose down

# Remove the volume
docker volume rm cracked-pillars_hammer_data

# Restart services
docker-compose up --build
```

### Build issues
If you encounter build issues:

```bash
# Clean Docker cache
docker system prune -a

# Rebuild without cache
docker-compose build --no-cache
```

## Production Considerations

1. **Security**: The current setup is for development. For production:
   - Use proper secrets management
   - Implement HTTPS
   - Add reverse proxy (nginx)
   - Configure proper logging

2. **Performance**: Consider:
   - Using a production database (PostgreSQL, MySQL)
   - Implementing caching layers
   - Adding load balancers

3. **Monitoring**: Add:
   - Health checks
   - Metrics collection
   - Log aggregation

## File Structure

```
cracked-pillars/
├── docker-compose.yml          # Main orchestration file
├── chisel/
│   ├── Dockerfile             # Frontend container
│   └── .dockerignore
├── hammer/
│   ├── Dockerfile             # Backend container
│   ├── .dockerignore
│   └── hammer.toml            # Configuration
└── DOCKER_DEPLOYMENT.md       # This file
```
