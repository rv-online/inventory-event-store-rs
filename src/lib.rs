pub mod domain;
pub mod store;

#[cfg(test)]
mod tests {
    use crate::domain::InventoryEventKind;
    use crate::store::{AppendError, InventoryStore};

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
}
