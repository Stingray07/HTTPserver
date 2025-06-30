# Async HTTP Server in Rust

A high-performance, asynchronous HTTP/1.1 server built from scratch with Rust and Tokio, featuring a modular architecture and robust error handling.

## 🚀 Features
- Asynchronous I/O using Tokio runtime for high concurrency
- Modular Design with clear separation of concerns
- HTTP/1.1 protocol support
- Request Routing with support for different HTTP methods and paths
- Static File Serving from the static directory
- JSON API endpoints with proper content negotiation
- Graceful Shutdown on interrupt signals
- Configurable Timeouts for requests and responses

## 🚦 Getting Started

### Prerequisites
- Rust 1.60+
- Cargo (Rust's package manager)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Stingray07/HTTPserver
   cd HTTPserver
2. Build the project:
   ```bash
   cargo build --release
### Running the Server:
    cargo run --release

## 📚 API Documentation

### Web Routes
- `GET /` - Home page
- `GET /about` - About page
- `GET /submit` - Form submission page
- `POST /submit/json` - Handle JSON form submissions
- `POST /submit/text` - Handle text form submissions
- `POST /submit/binary` - Handle binary form submissions
- `GET /chunky` - Endpoint for testing chunked transfer encoding
- `GET /static/*` - Serve static files (handled by [serve_file](cci:1://file:///c:/Users/Stingray/Desktop/HTTP%20server%20project/http_serverrrrr/src/http_utils/response.rs:11:0-47:1))

### API Endpoints (v1)

#### Users
- `GET /api/v1/users` - Get user information
  - Query Parameters: Defined in `query_map`

#### Posts
- `POST /api/v1/posts` - Create a new post
  - Query Parameters: Defined in `query_map`
  - Body: Post data

### Error Handling
- `400` - Bad Request (`/400`)
- `403` - Forbidden (Returned for invalid paths)
- `404` - Not Found (Default for undefined routes)

### Request/Response Format
- **Content-Type**: 
  - API responses: `application/json`
  - Web routes: `text/html`
  - Static files: Auto-detected MIME type

## License

[MIT](https://choosealicense.com/licenses/mit/)



