use crate::semiology::referent::Referent;

/// An item is an object that takes the place of slots.
/// field `sign` stores possibly useful data
pub struct Item {
    sign: u16,
    refer: Box<dyn Referent>,
}

pub struct ItemPack {
    bound: usize,
    items: Vec<Item>,
}

impl ItemPack {
    fn clear(&mut self) {
        self.items.clear();
    }
    fn push(&mut self, it: Item) -> bool {
        if (self.items.len() < self.bound) {
            self.items.push(it);
            return true;
        }
        return false;
    }
}
