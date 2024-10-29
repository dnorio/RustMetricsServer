# Metrics Server

This project is a Rust-based metrics server designed to handle various data processing tasks, including:

- Processing Parquet and ORC files.
- Collecting and querying data from Elasticsearch.
- Copying and storing data in PostgreSQL.

The server is optimized for performance, with Docker setup for easy deployment and debugging.

## Features

- **Parquet & ORC Processing**: Handle large-scale data stored in Parquet or ORC formats.
- **Elasticsearch Integration**: Collect and analyze data from Elasticsearch indices.
- **PostgreSQL Support**: Copy and store processed data in PostgreSQL for analytics and reporting.
- **Dockerized**: Containerized for quick deployment, debugging, and scaling.

## Getting Started

### Prerequisites

- **Rust**: Make sure Rust and Cargo are installed. Visit [Rust's official site](https://www.rust-lang.org/) for instructions.
- **Docker**: Install Docker and Docker Compose for containerization.

### Project Setup

1. **Clone the repository**:

```bash
git clone https://github.com/dnorio/RustMetricsServer.git
cd RustMetricsServer
```

2. **Build the project**:

```bash
cargo build --release
```

3. **Run the server**:

```bash
cargo run
```

### Docker Setup

The project includes a Docker setup for easy deployment. Follow these steps to run the server within a Docker container.

1. **Build the Docker image**:

```bash
  docker-compose build
```

2. **Run the server**:

```bash
docker-compose up
```

The server should now be accessible on http://localhost:3000.

### Development with Docker

For live reloading during development, use the following command:

```bash
docker-compose up --build
```

This setup uses _cargo-watch_ to monitor changes and recompile automatically.

### Debugging

To debug within the Docker container:

1. Expose the debug port in docker-compose.yml.
2. Attach your debugger to port 9229.

## Project Structure

```plaintext
RustMetricsServer/
├── Dockerfile # Multi-stage Docker build
├── docker-compose.yml # Docker Compose setup
├── src/
│ └── main.rs # Main server logic
├── Cargo.toml # Project dependencies
└── .gitignore # Git ignored files
```

### File Breakdown

- main.rs: Initializes the HTTP server, routes, and handles data processing.
- Dockerfile: Multi-stage build for efficient compilation and deployment.
- docker-compose.yml: Defines the Docker setup with live-reloading and debugging options.

## Future Enhancements

- Advanced Analytics: Add support for additional data analytics features.
- Enhanced Logging: Integrate structured JSON logging.
- Authentication: Implement route-based authentication.

## Contributing

Contributions are welcome! Please submit issues and pull requests.
