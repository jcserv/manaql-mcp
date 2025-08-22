# MTG MCP Server

A Model Context Protocol (MCP) server for Magic: The Gathering card data, built with Rust and Actix Web.

## Features

- RESTful API for MTG card data
- Support for cards and printings
- PostgreSQL database integration
- Search and pagination support

## API Endpoints

### Cards

- `GET /api/v1/cards` - List all cards (with pagination)
- `GET /api/v1/cards/{id}` - Get a specific card by ID
- `GET /api/v1/cards/name/{name}` - Get a card by name
- `GET /api/v1/cards/search?q={query}` - Search cards by name
- `POST /api/v1/cards` - Create a new card
- `PATCH /api/v1/cards/{id}` - Update a card
- `DELETE /api/v1/cards/{id}` - Delete a card

### Printings

- `GET /api/v1/printings` - List all printings (with pagination)
- `GET /api/v1/printings/{id}` - Get a specific printing by ID
- `GET /api/v1/printings/card/{card_id}` - Get all printings for a card
- `GET /api/v1/printings/set/{set_code}` - Get all printings for a set
- `POST /api/v1/printings` - Create a new printing
- `PATCH /api/v1/printings/{id}` - Update a printing
- `DELETE /api/v1/printings/{id}` - Delete a printing

## Query Parameters

- `limit` - Number of items to return (default: 100)
- `offset` - Number of items to skip (default: 0)

## Database Schema

The server connects to a PostgreSQL database with the following tables:

- `card` - Basic card information (id, name, main_type)
- `printing` - Card printings with set information, prices, and images

## Setup

1. Set the `DATABASE_URL` environment variable
2. Run `cargo run` to start the server
3. The server will be available at `http://localhost:8000`

## Development

- `cargo check` - Check for compilation errors
- `cargo test` - Run tests
- `cargo build` - Build the project