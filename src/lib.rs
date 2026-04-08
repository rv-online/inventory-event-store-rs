pub mod domain;
pub mod store;

#[cfg(test)]
mod tests {
    use crate::domain::{InventoryEventKind, InventoryHealth};
    use crate::store::{AppendError, InventoryStore, StoreMetrics};

    #[test]
    fn applies_inventory_lifecycle_and_builds_snapshot() {
        let mut store = InventoryStore::default();
        let received = store
            .append("sku-1", 0, InventoryEventKind::Received, 10)
            .unwrap();
        assert_eq!(received.available, 10);

        let reserved = store
            .append("sku-1", 1, InventoryEventKind::Reserved, 4)
            .unwrap();
        assert_eq!(reserved.available, 6);

        let shipped = store
            .append("sku-1", 2, InventoryEventKind::Shipped, 3)
            .unwrap();
        assert_eq!(shipped.on_hand, 7);
        assert_eq!(shipped.reserved, 1);
        assert_eq!(shipped.shipped, 3);
    }

    #[test]
    fn rejects_version_conflicts() {
        let mut store = InventoryStore::default();
        store
            .append("sku-2", 0, InventoryEventKind::Received, 5)
            .unwrap();

        let result = store.append("sku-2", 0, InventoryEventKind::Reserved, 2);
        assert_eq!(
            result,
            Err(AppendError::VersionConflict {
                expected: 0,
                actual: 1,
            })
        );
    }

    #[test]
    fn rejects_invalid_shipments() {
        let mut store = InventoryStore::default();
        store
            .append("sku-3", 0, InventoryEventKind::Received, 2)
            .unwrap();
        let result = store.append("sku-3", 1, InventoryEventKind::Shipped, 1);
        assert!(matches!(result, Err(AppendError::InvalidEvent(_))));
    }

    #[test]
    fn builds_low_stock_projection() {
        let mut store = InventoryStore::default();
        store
            .append("sku-4", 0, InventoryEventKind::Received, 10)
            .unwrap();
        store
            .append("sku-4", 1, InventoryEventKind::Reserved, 7)
            .unwrap();
        store
            .append("sku-4", 2, InventoryEventKind::Shipped, 5)
            .unwrap();

        let projection = store.projection("sku-4", 3).unwrap();
        assert_eq!(projection.health, InventoryHealth::LowStock);
        assert_eq!(projection.available, 3);
        assert!(projection.fill_rate > 0.7);
    }

    #[test]
    fn rebuilds_from_snapshot_plus_remaining_events() {
        let mut store = InventoryStore::default();
        store
            .append("sku-5", 0, InventoryEventKind::Received, 12)
            .unwrap();
        let snapshot = store
            .append("sku-5", 1, InventoryEventKind::Reserved, 5)
            .unwrap();
        store
            .append("sku-5", 2, InventoryEventKind::Released, 2)
            .unwrap();
        store
            .append("sku-5", 3, InventoryEventKind::Shipped, 2)
            .unwrap();

        let rebuilt = store
            .rebuild_from_snapshot(&snapshot, &store.stream("sku-5"))
            .unwrap();
        assert_eq!(rebuilt.on_hand, 10);
        assert_eq!(rebuilt.reserved, 1);
        assert_eq!(rebuilt.shipped, 2);
    }

    #[test]
    fn summarizes_store_metrics() {
        let mut store = InventoryStore::default();
        store
            .append("sku-a", 0, InventoryEventKind::Received, 4)
            .unwrap();
        store
            .append("sku-b", 0, InventoryEventKind::Received, 10)
            .unwrap();
        store
            .append("sku-a", 1, InventoryEventKind::Reserved, 2)
            .unwrap();

        let metrics = store.metrics(3).unwrap();
        assert_eq!(
            metrics,
            StoreMetrics {
                tracked_skus: 2,
                total_events: 3,
                low_stock_skus: 1,
            }
        );
    }
}
