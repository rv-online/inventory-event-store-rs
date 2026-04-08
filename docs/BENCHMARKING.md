# Benchmarking Notes

## Current State

This repository currently validates with `cargo check` on this machine. Full `cargo test` and executable benchmarking require the MSVC linker (`link.exe`), which is not available in the current environment.

## Intended Benchmark Path

Once the linker is installed, a useful benchmark pass would measure:

1. append throughput per SKU stream
2. aggregate rebuild latency as event counts grow
3. snapshot-assisted recovery versus full replay

## Why This Matters

Event-sourced systems often look elegant in code but fall apart operationally when replay paths and concurrency checks are not measured. Even for a portfolio project, showing the benchmark plan makes the engineering intent clearer.
