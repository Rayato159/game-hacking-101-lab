use bevy::prelude::*;

pub trait Food {
    fn eat(&self);
    fn get_value(&self) -> i32;
}

#[derive(Component)]
pub struct PadKaprow {
    pub health: i32,
}

impl Food for PadKaprow {
    fn eat(&self) {
        info!("Eat Pad Kaprow: health: {}", self.health);
    }

    fn get_value(&self) -> i32 {
        self.health
    }
}

#[derive(Component)]
pub struct Coffee {
    pub mana: i32,
}

impl Food for Coffee {
    fn eat(&self) {
        info!("Drink coffee: mana: {}", self.mana);
    }

    fn get_value(&self) -> i32 {
        self.mana
    }
}
