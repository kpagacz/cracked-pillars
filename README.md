# cracked-pillars

A treasure trove of information about Pillars of Eternity II

## Project Structure

### Quarry (`/quarry`)

The data extraction and processing pipeline that builds our knowledge base.
This directory contains tools for:

- Scraping game data (abilities, items) from the official wiki
- Processing and structuring the data using LLMs
- Creating a searchable index of game mechanics
- Validating the processed data

See the [quarry README](quarry/README.md) for detailed information about
the data processing pipeline and usage instructions.

### Hammer (`/hammer`)

The backend API server that provides data access and search functionality.
This Rust-based service includes:

- RESTful API endpoints for items, abilities, and tags
- SQLite database with game data
- Search and filtering capabilities
- Data indexing for fast queries
- Import functionality from the quarry data

### Chisel (`/chisel`)

The frontend web application built with Next.js that provides the user interface.
This React-based application includes:

- Modern, responsive UI for browsing game data
- Search and filtering interface
- Item and ability detail pages
- Explore page with advanced filtering options
- Server-side rendering for optimal performance

### Docker Configuration

The project includes Docker configuration for containerized deployment:

- `docker-compose.yml` - Development environment setup
- `docker-compose.prod.yml` - Production environment configuration
- `Dockerfile` files in each service directory
- `build-images.sh` - Script for building Docker images
- `push-images-to-ecr.sh` - Script for deploying to AWS ECR

### Documentation

- `DOCKER_DEPLOYMENT.md` - Detailed deployment instructions
- `plan.md` - Project planning and roadmap
- `LICENSE` - Project license information
