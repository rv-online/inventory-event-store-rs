#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InventoryEventKind {
    Received,
    Reserved,
    Released,
    Shipped,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryEvent {
    pub sku: String,
    pub quantity: u32,
    pub sequence: u64,
    pub kind: InventoryEventKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct InventoryAggregate {
    pub sku: String,
    pub on_hand: u32,
    pub reserved: u32,
    pub shipped: u32,
    pub version: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventorySnapshot {
    pub sku: String,
    pub on_hand: u32,
    pub reserved: u32,
    pub available: u32,
    pub shipped: u32,
    pub version: u64,
}

impl InventoryAggregate {
    pub fn apply(&mut self, event: &InventoryEvent) -> Result<(), String> {
        if self.sku.is_empty() {
            self.sku = event.sku.clone();
        }

        match event.kind {
            InventoryEventKind::Received => {
                self.on_hand = self.on_hand.saturating_add(event.quantity);
            }
            InventoryEventKind::Reserved => {
                if self.available() < event.quantity {
                    return Err(format!("insufficient inventory for sku {}", event.sku));
                }
                self.reserved = self.reserved.saturating_add(event.quantity);
            }
            InventoryEventKind::Released => {
                if self.reserved < event.quantity {
                    return Err(format!("cannot release more than reserved for sku {}", event.sku));
                }
                self.reserved -= event.quantity;
            }
            InventoryEventKind::Shipped => {
                if self.reserved < event.quantity || self.on_hand < event.quantity {
                    return Err(format!("cannot ship unavailable inventory for sku {}", event.sku));
                }
                self.reserved -= event.quantity;
                self.on_hand -= event.quantity;
                self.shipped = self.shipped.saturating_add(event.quantity);
            }
        }

        self.version = event.sequence;
        Ok(())
    }

    pub fn available(&self) -> u32 {
        self.on_hand.saturating_sub(self.reserved)
    }

    pub fn to_snapshot(&self) -> InventorySnapshot {
        InventorySnapshot {
            sku: self.sku.clone(),
            on_hand: self.on_hand,
            reserved: self.reserved,
            available: self.available(),
            shipped: self.shipped,
            version: self.version,
        }
    }
}
