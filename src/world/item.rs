use super::*;

#[derive(Default, Debug, Clone)]
pub struct HeldItems {
    pub left_hand: Option<Item>,
    pub right_hand: Option<Item>,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub on_use: ActionEffect,
}

pub type ItemId = HandId;

#[derive(Debug, Clone, Copy)]
pub enum HandId {
    LeftHand,
    RightHand,
}

impl HeldItems {
    pub fn get_any_item(&self) -> Option<&Item> {
        self.left_hand.as_ref().or(self.right_hand.as_ref())
    }

    pub fn get_item(&self, id: ItemId) -> Option<&Item> {
        match id {
            ItemId::LeftHand => self.left_hand.as_ref(),
            ItemId::RightHand => self.right_hand.as_ref(),
        }
    }

    pub fn get_item_mut(&mut self, id: ItemId) -> Option<&mut Item> {
        match id {
            ItemId::LeftHand => self.left_hand.as_mut(),
            ItemId::RightHand => self.right_hand.as_mut(),
        }
    }

    pub fn get_hand_mut(&mut self, id: HandId) -> &mut Option<Item> {
        match id {
            HandId::LeftHand => &mut self.left_hand,
            HandId::RightHand => &mut self.right_hand,
        }
    }
}

impl Item {
    pub fn sword() -> Self {
        Self {
            on_use: ActionEffect::MeleeAttack {
                damage: Hp::new(1.0),
            },
        }
    }

    // pub fn shield() -> Self {
    //     todo!()
    // }

    // pub fn bow() -> Self {
    //     Self {
    //         on_use: ActionEffect::
    //     }
    // }
}
