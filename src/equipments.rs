//use std::cmp;

#[derive(Debug)]
pub enum ArmorType { Heavy }

#[derive(Debug)]
pub struct Armor {
    armor_type:ArmorType,
    armor_class:i32
}

impl Armor {

    /*
    pub fn create_leather() -> Self {
        Armor { armor_type: ArmorType::Light, armor_class: 11 }
    }

    pub fn create_chain_shirt() -> Self {
        Armor { armor_type: ArmorType::Medium, armor_class: 13 }
    }
    */

    pub fn create_chain_mail() -> Self {
        Armor { armor_type: ArmorType::Heavy, armor_class: 16 }
    }

    pub fn get_armor_class(&self, _dex_mod:i32) -> i32 {
        match self.armor_type {
            //ArmorType::Light => self.get_light_ac(dex_mod),
            //ArmorType::Medium => self.get_medium_ac(dex_mod),
            ArmorType::Heavy => self.get_heavy_ac()
        }
    }

    /*
    fn get_light_ac(&self, dex_mod:i32) -> i32 {
        self.armor_class + dex_mod
    }

    fn get_medium_ac(&self, dex_mod:i32) -> i32 {
        self.armor_class + cmp::min(2, dex_mod)
    }
    */

    fn get_heavy_ac(&self) -> i32 {
        self.armor_class
    }

}