# Bio MCP Server

A Rust-based MCP (Machine-Checked Protocol) server for bioinformatics.

This server is designed to be used with AI assistants like Cursor to provide access to bioinformatics APIs and tools.

## Building

To build the project, run:

```bash
cargo build
```

## Running

To run the server, use:

```bash
cargo run
```

The server listens on standard input for MCP messages and sends responses to standard output. 