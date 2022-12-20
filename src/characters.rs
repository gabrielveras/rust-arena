
use std::cmp;
use std::vec;

use crate::actions;
use crate::actions::Action;
use crate::equipments;
use crate::rules;

#[derive(Clone, Debug)]
pub struct BaseCharacter {
    experience: i32,
    hit_dice: i32,
    hit_die_size: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    rolled_hit_points: i32,
    max_hit_points: i32,
    current_hp: i32,
    is_alive: bool,
    pub armor_class: i32,
    pub armor: equipments::Armor,
    pub proficiency: i32,
    pub actions: Vec<Box<actions::MeleeWeaponAttack>>
}

impl BaseCharacter {

    pub fn new_fighter() -> Self {
        BaseCharacter::new(10)
    }

    fn new(hit_die_size:i32) -> Self {
        let mut character = Self {
            experience: 0,
            hit_dice: 1,
            hit_die_size: hit_die_size,
            strength: rules::roll_sum(4, 6, true),
            dexterity: rules::roll_sum(4, 6, true),
            constitution: rules::roll_sum(4, 6, true),
            rolled_hit_points: 0,
            max_hit_points: 0,
            current_hp: 0,
            is_alive: true,
            armor_class: 0,
            armor: equipments::Armor::create_chain_mail(),
            proficiency: 0,
            actions: vec![]
        };
        character.update();
        character.current_hp = character.max_hit_points;
        character
    }

    pub fn add_action(&mut self, action:Box<actions::MeleeWeaponAttack>) {
        self.actions.push(action);
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn long_rest(&mut self) {
        for i in 0..self.actions.len() {
            self.actions[i].long_rested = true;
        }
    }

    pub fn take_damage (&mut self, damage: i32) {
        self.current_hp -= damage;
        self.current_hp = cmp::max(self.current_hp, 0);
        self.is_alive = self.current_hp > 0
    }

    pub fn update(&mut self) {
        self.proficiency = rules::get_proficiency(self.hit_dice);
        self.armor_class = self.armor.get_armor_class(rules::get_ability_modifier(self.dexterity));
        self.max_hit_points = rules::get_max_hit_points(self.hit_dice, self.rolled_hit_points, self.hit_die_size, self.constitution);
        for i in 0..self.actions.len() {
            self.actions[i].clone().update(self);
        }
    }

}

#[derive(Clone, Debug)]
pub struct BaseParty {
    pub members:Vec<Box<BaseCharacter>>,
    pub fallen:Vec<Box<BaseCharacter>>
}

impl BaseParty {

    pub fn new() -> Self {
        let mut _members = vec![];
        let mut _fallen = vec![];
        Self {
            members: _members,
            fallen: _fallen
        }
    }

    pub fn add_experience(&mut self, amount:i32) {
        for i in 0..self.members.len() {
            self.members[i].experience += amount;
        }
    }

    pub fn any_alive(&self) -> bool {
        self.members.iter().any(|x| x.is_alive())
    }

    pub fn fall_members(&mut self) {
        let mut i = 0;
        while i < self.members.len() {
            if !self.members[i].is_alive() {
                let dead_member = self.members.remove(i);
                self.fallen.push(dead_member);
            } else {
                i += 1;
            }
        }
    }

    pub fn get_xp_from_fallen(&self) -> i32 {
        let mut xp = 0;
        for i in 0..self.fallen.len() {
            xp += self.fallen[i].hit_dice * rules::XP_PER_CHAR_LEVEL;
        }
        xp
    }

    pub fn long_rest(&mut self) {
        for i in 0..self.members.len() {
            self.members[i].long_rest()
        }
    }

    pub fn remove_dead(&mut self) {
    }

}