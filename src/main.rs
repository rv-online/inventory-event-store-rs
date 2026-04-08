use inventory_event_store_rs::domain::InventoryEventKind;
use inventory_event_store_rs::store::InventoryStore;

fn main() {
    let mut store = InventoryStore::default();
    store
        .append("sku-demo", 0, InventoryEventKind::Received, 25)
        .unwrap();
    store
        .append("sku-demo", 1, InventoryEventKind::Reserved, 8)
        .unwrap();
    store
        .append("sku-demo", 2, InventoryEventKind::Shipped, 5)
        .unwrap();

    let snapshot = store.snapshot("sku-demo").unwrap();
    println!(
        "sku={} version={} on_hand={} reserved={} available={} shipped={}",
        snapshot.sku,
        snapshot.version,
        snapshot.on_hand,
        snapshot.reserved,
        snapshot.available,
        snapshot.shipped
    );
}
