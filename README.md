# Inventory Event Store RS

Rust event-sourced inventory engine with aggregate rebuilding, snapshots, and optimistic concurrency enforcement.

## Why This Exists

Shaped like the core of a warehouse or fulfillment system where event history is authoritative and conflicting writes must be rejected cleanly.

## What This Demonstrates

- event sourcing fundamentals with aggregate replay
- optimistic concurrency checks on stream appends
- snapshot generation for read-side consumption

## Architecture

1. inventory events are appended to per-SKU streams
1. aggregates are rebuilt from streams and projected into snapshots
1. append operations reject invalid transitions and version conflicts

## Run It

```bash
cargo check
cargo test
cargo run
```

## Verification

Use `cargo check` for compile validation on this machine. `cargo test` is expected to work once the MSVC linker (`link.exe`) is installed.
