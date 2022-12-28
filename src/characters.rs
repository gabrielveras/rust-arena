
use std::cmp;
use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::vec;

use crate::actions;
use crate::equipments;
use crate::rules;
use crate::tables;

const EXPERIENCE_TABLE:&[i32; 20] = &[0, 300, 900, 2700, 6500, 14000, 23000, 34000, 48000, 64000, 85000, 100000, 120000, 140000, 165000, 195000, 225000, 265000, 305000, 355000];

pub struct CharacterData {
    pub name: String,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
}

pub struct BaseCharacter {
    experience: i32,
    hit_dice: i32,
    hit_die_size: i32,
    rolled_hit_points: i32,
    max_hit_points: i32,
    current_hp: i32,
    is_alive: bool,
    pub armor_class: i32,
    pub armor: equipments::Armor,
    pub proficiency: i32,
    pub data: CharacterData,
    pub actions: Vec<Box<actions::Action>>
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
            rolled_hit_points: 0,
            max_hit_points: 0,
            current_hp: 0,
            is_alive: true,
            armor_class: 0,
            armor: equipments::Armor::create_chain_mail(),
            proficiency: 0,
            actions: vec![],
            data: CharacterData{
                name: tables::random_name(),
                strength: rules::roll_sum(4, 6, true),
                dexterity: rules::roll_sum(4, 6, true),
                constitution: rules::roll_sum(4, 6, true),
            },
        };
        character.update();
        character.current_hp = character.max_hit_points;
        character
    }

    pub fn add_action(&mut self, action:Box<actions::Action>) {
        self.actions.push(action);
    }

    fn add_level(&mut self) {
        if self.hit_dice < 20 {
            self.hit_dice += 1;
            self.rolled_hit_points += rules::roll_die(self.hit_die_size);
            self.update();
        }
    }

    pub fn add_experience(&mut self, amount:i32) {
        self.experience += amount;
        if self.experience >= EXPERIENCE_TABLE[self.hit_dice as usize] {
            self.add_level();
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn long_rest(&mut self) {
        if self.current_hp > 0 {
            self.current_hp = self.max_hit_points;
            for action in &mut self.actions {
                action.long_rest();
            }
        }
    }

    pub fn take_damage (&mut self, damage: i32) {
        self.current_hp -= damage;
        self.current_hp = cmp::max(self.current_hp, 0);
        self.is_alive = self.current_hp > 0;
    }

    pub fn update(&mut self) {
        self.proficiency = rules::get_proficiency(self.hit_dice);
        self.armor_class = self.armor.get_armor_class(rules::get_ability_modifier(self.data.dexterity));
        self.max_hit_points = rules::get_max_hit_points(self.hit_dice, self.rolled_hit_points, self.hit_die_size, self.data.constitution);
        for action in &mut self.actions {
            action.update(&self.data);
        }
    }

}

impl Display for BaseCharacter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <13} [ HD {: >2} HP {: >3} AC {: >2} Str {: >2} Con {: >2} Dex {: >2} ]",
            self.data.name,
            self.hit_dice,
            self.max_hit_points,
            self.armor_class,
            self.data.strength,
            self.data.dexterity,
            self.data.constitution,
        )
    }
}

pub struct BaseParty {
    pub members:VecDeque<Box<BaseCharacter>>,
    pub fallen:Vec<Box<BaseCharacter>>
}

impl BaseParty {

    pub fn new() -> Self {
        let mut _members = VecDeque::new();
        let mut _fallen = vec![];
        Self {
            members: _members,
            fallen: _fallen
        }
    }

    pub fn add_experience(&mut self, amount:i32) {
        if self.members.len() > 0 {
            let amount_per_member = amount / (self.members.len() as i32);
            for member in &mut self.members {
                member.add_experience(amount_per_member);
            }
        }
    }

    pub fn any_alive(&self) -> bool {
        self.members.iter().any(|x| x.is_alive())
    }

    pub fn fall_members(&mut self) {
        let mut i = 0;
        while i < self.members.len() {
            if !self.members[i].is_alive() {
                match self.members.remove(i) {
                    Some(dead) => self.fallen.push(dead),
                    None => (),
                }
            } else {
                i += 1;
            }
        }
    }

    pub fn get_xp_from_fallen(&mut self) -> i32 {
        let mut xp = 0;
        for dead in &mut self.fallen {
            xp += dead.hit_dice * rules::XP_PER_CHAR_LEVEL;
        }
        xp
    }

    pub fn long_rest(&mut self) {
        for member in &mut self.members {
            member.long_rest()
        }
    }

    pub fn remove_dead(&mut self) {
    }

}

impl Display for BaseParty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let members_len = self.members.len();
        if members_len == 0 {
            write!(f, "Dead party")
        } else {
            if members_len > 1 {
                write!(f, "{}({}) [HP {}/{}]",
                    self.members[0].data.name,
                    members_len,
                    self.members[0].current_hp, self.members[0].max_hit_points)
            } else {
                write!(f, "{} [HP {}/{}]",
                    self.members[0].data.name,
                    self.members[0].current_hp,
                    self.members[0].max_hit_points)
            }
        }
    }
}