# ManaQL MCP Server

A Model Context Protocol (MCP) server for Magic: The Gathering card management, built with Rust and the `rmcp` SDK.

## Architecture
```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library exports
├── error.rs             # Error handling
├── cards/               # Cards domain module
│   ├── mod.rs
│   ├── model.rs         # Card data models
│   ├── repository.rs    # Database operations
│   └── mcp.rs           # Cards MCP functionality
└── mcp/                 # Generic MCP server implementation
    ├── mod.rs           # MCP module exports
    └── server.rs        # Main MCP server with tools/prompts
```

## Features

### Tools
- **Card Search**: Search for cards by name
- **Card Retrieval**: Get specific cards by ID
- **Type Filtering**: Get cards by type
- **Card Count**: Get total number of cards in database

### Prompts
- TODO

## Usage

### Running with stdio transport
```bash
cargo run
```

## Development

### Testing
```bash
cargo test
```

### Building
```bash
cargo build --release
```

### Using with MCP Inspector
```bash
npx @modelcontextprotocol/inspector cargo run
```