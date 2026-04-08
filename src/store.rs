use std::collections::HashMap;

use crate::domain::{
    InventoryAggregate, InventoryEvent, InventoryEventKind, InventoryProjection, InventorySnapshot,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AppendError {
    VersionConflict { expected: u64, actual: u64 },
    InvalidEvent(String),
}

#[derive(Default)]
pub struct InventoryStore {
    streams: HashMap<String, Vec<InventoryEvent>>,
    snapshots: HashMap<String, InventorySnapshot>,
}

#[derive(Debug, PartialEq)]
pub struct StoreMetrics {
    pub tracked_skus: usize,
    pub total_events: usize,
    pub low_stock_skus: usize,
}

impl InventoryStore {
    pub fn append(
        &mut self,
        sku: &str,
        expected_version: u64,
        kind: InventoryEventKind,
        quantity: u32,
    ) -> Result<InventorySnapshot, AppendError> {
        let current_version = self
            .streams
            .get(sku)
            .and_then(|events| events.last())
            .map(|event| event.sequence)
            .unwrap_or(0);

        if current_version != expected_version {
            return Err(AppendError::VersionConflict {
                expected: expected_version,
                actual: current_version,
            });
        }

        let event = InventoryEvent {
            sku: sku.to_string(),
            quantity,
            sequence: current_version + 1,
            kind,
        };

        let mut aggregate = self.rebuild(sku).map_err(AppendError::InvalidEvent)?;
        aggregate.apply(&event).map_err(AppendError::InvalidEvent)?;
        self.streams.entry(sku.to_string()).or_default().push(event);
        let snapshot = aggregate.to_snapshot();
        self.snapshots.insert(sku.to_string(), snapshot.clone());
        Ok(snapshot)
    }

    pub fn rebuild(&self, sku: &str) -> Result<InventoryAggregate, String> {
        let mut aggregate = InventoryAggregate::default();
        if let Some(events) = self.streams.get(sku) {
            for event in events {
                aggregate.apply(event)?;
            }
        }
        Ok(aggregate)
    }

    pub fn snapshot(&self, sku: &str) -> Option<InventorySnapshot> {
        self.snapshots.get(sku).cloned()
    }

    pub fn stream(&self, sku: &str) -> Vec<InventoryEvent> {
        self.streams.get(sku).cloned().unwrap_or_default()
    }

    pub fn projection(&self, sku: &str, reorder_point: u32) -> Result<InventoryProjection, String> {
        self.rebuild(sku).map(|aggregate| aggregate.to_projection(reorder_point))
    }

    pub fn metrics(&self, reorder_point: u32) -> Result<StoreMetrics, String> {
        let mut low_stock_skus = 0;
        for sku in self.streams.keys() {
            let projection = self.projection(sku, reorder_point)?;
            if projection.available <= reorder_point {
                low_stock_skus += 1;
            }
        }

        Ok(StoreMetrics {
            tracked_skus: self.streams.len(),
            total_events: self.streams.values().map(|events| events.len()).sum(),
            low_stock_skus,
        })
    }

    pub fn rebuild_from_snapshot(
        &self,
        snapshot: &InventorySnapshot,
        remaining_events: &[InventoryEvent],
    ) -> Result<InventoryAggregate, String> {
        let mut aggregate = InventoryAggregate {
            sku: snapshot.sku.clone(),
            on_hand: snapshot.on_hand,
            reserved: snapshot.reserved,
            shipped: snapshot.shipped,
            version: snapshot.version,
        };

        for event in remaining_events {
            if event.sequence <= snapshot.version {
                continue;
            }
            aggregate.apply(event)?;
        }

        Ok(aggregate)
    }
}
