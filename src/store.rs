use std::collections::HashMap;

use crate::domain::{InventoryAggregate, InventoryEvent, InventoryEventKind, InventorySnapshot};

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
}
