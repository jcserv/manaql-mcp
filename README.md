# ManaQL MCP Server

A Model Context Protocol (MCP) server for Magic: The Gathering card data, providing tools for searching and analyzing MTG cards.

## Features

- **Card Search**: Search cards by name, type, and other filters
- **Card Lookup**: Get specific cards by ID
- **Card Count**: Get total number of cards in database
- **Vector Similarity Search**: Find similar cards using AI embeddings based on characteristics like type, mana cost, function, etc.

## Tools

### `search_cards`
Search for cards using filters (name, type) and optional query for additional filtering across multiple fields with pagination support.

### `get_card_by_id`
Get a specific card by ID.

### `get_card_count`
Get total number of cards in database.

### `find_similar_cards`
Find similar cards using vector similarity search based on card characteristics like type, mana cost, function, etc.

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
- **Card Search**: Search for cards by name, type
- **Card Retrieval**: Get specific cards by ID
- **Card Count**: Get total number of cards in database
- **Get Similar Cards**: Get cards similar to another card

### Prompts
- TODO

## Usage

1. Setup the ManaQL DB - follow the instructions [here](https://github.com/jcserv/manaql-ingest)
2. Run the model context protocol inspector
```bash
npx @modelcontextprotocol/inspector cargo run
```