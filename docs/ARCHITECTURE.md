# Architecture

## Goal

This crate models a compact event-sourced inventory core with enough operational behavior to feel like a fulfillment subsystem instead of a toy example.

## Flow

1. Commands append typed inventory events to a per-SKU stream.
2. The aggregate replays those events to derive on-hand, reserved, shipped, and available counts.
3. Snapshots expose a fast read model for current inventory state.
4. Projections add operator-facing context such as low-stock health and fill-rate posture.

## Design Tradeoffs

- The write model is intentionally in-memory so the event-sourcing mechanics stay easy to inspect.
- Optimistic concurrency is explicit at append time instead of hidden behind storage behavior.
- Projection logic lives beside the aggregate so the read-model story is visible without introducing another crate.
