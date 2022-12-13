use std::cmp;

use crate::rules;

pub struct BaseCharacter {
    hit_dice: i32,
    experience: i32,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    rolled_hit_points: i32,
    max_hit_points: i32,
    current_hp: i32,
    is_alive: bool,
    armor_class: i32
}

impl BaseCharacter {

    pub fn new () -> Self {
        Self {
            experience: 0,
            strength: 10,
            dexterity: 10,
            constitution: 10,
            max_hit_points: 0,
            current_hp: 0,
            is_alive: false,
            hit_dice: 0,
            rolled_hit_points: 0,
            armor_class: 10
        }
    }

    fn update (&mut self) {
        self.set_armor_class();
        self.max_hit_points = rules::get_max_hit_points(self.hit_dice, self.rolled_hit_points, 8 /* TODO: self.__class__.HIT_DIE_SIZE */, self.constitution)
    }

    fn take_damage (&mut self, damage: i32) {
        self.current_hp -= damage;
        self.current_hp = cmp::max(self.current_hp, 0);
        self.is_alive = self.current_hp > 0
    }

    fn set_armor_class(&mut self) {
        self.armor_class = 10
    }

}
