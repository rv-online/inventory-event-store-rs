# Inventory Event Store RS

Rust project that models event-sourced inventory with optimistic concurrency, snapshots, and aggregate rebuilding. It is meant to read like the core of a warehouse or fulfillment service rather than a toy crate.

## What It Shows

- event sourcing fundamentals
- optimistic concurrency control
- aggregate rebuilding from event streams
- snapshot generation for faster reads
- Rust domain modeling and test coverage

## Commands

```bash
cargo test
cargo run
```
