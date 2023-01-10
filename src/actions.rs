use std::cmp;

use crate::characters::{ BaseCharacter, CharacterData };
use crate::rules;
use crate::console;
use crate::equipments::weapon::{ WeaponProperty };

pub enum Action {
    MeleeWeaponAttack(MeleeWeaponAttack)
}

impl Action {
    pub fn execute(&self, ally: &BaseCharacter, enemy:&mut BaseCharacter) {
        match self {
            Action::MeleeWeaponAttack(ctx) => execute_melee_weapon_attack(ctx, ally, enemy),
        }
    }
    pub fn update(&mut self, owner_data:&CharacterData) {
        match self {
            Action::MeleeWeaponAttack(ctx) => update_melee_weapon_attack(ctx, owner_data),
        }
    }
    pub fn long_rest(&mut self) {
        match self {
            Action::MeleeWeaponAttack(ctx) => long_rest_melee_weapon_attack(ctx),
        }
    }
}

pub struct MeleeWeaponAttack {
    pub name: String,
    pub ability_modifier: i32,
    pub damage_dice: (i32,i32),
    pub properties: WeaponProperty,
    pub long_rested: bool,
}

impl MeleeWeaponAttack {
    pub fn new (name:String, damage_dice:(i32,i32), properties:WeaponProperty) -> Box<Action> {
        let obj = Self {
            name: name,
            ability_modifier: 0,
            damage_dice: damage_dice,
            properties: properties,
            long_rested: true
        };
        Box::new(Action::MeleeWeaponAttack(obj))
    }
}

fn update_melee_weapon_attack(ctx:&mut MeleeWeaponAttack, owner:&CharacterData) {
    let strength_mod = rules::get_ability_modifier(owner.strength);
    let finesse_mod = cmp::max(strength_mod, rules::get_ability_modifier(owner.dexterity));
    //let use_finesse = bool(WeaponProperty.FINESSE in self.weapon_properties)
    let use_finesse = false;
    ctx.ability_modifier = strength_mod * (!use_finesse as i32) + finesse_mod * (use_finesse as i32);
}

fn execute_melee_weapon_attack(ctx:&MeleeWeaponAttack, ally:&BaseCharacter, enemy:&mut BaseCharacter) {
    if console::VERBOSE_ACTIONS { print!("{} tries to hit {} with {} in melee. ", ally.data.name, enemy.data.name, ctx.name); }
    let die_roll = rules::roll_die(20);
    if die_roll > 1 {
        let attack_roll = die_roll + ally.proficiency + ctx.ability_modifier;
        if die_roll == 20 || attack_roll >= enemy.armor_class {
            let dmg_dice = ctx.damage_dice.1 + 2*(ctx.properties.contains(WeaponProperty::VERSATILE) as i32);
            let damage = rules::roll_sum(ctx.damage_dice.0, dmg_dice, false)
                + ctx.ability_modifier;
                //+ (die_roll >= self.owner.skill_critical_hit) * rules::roll_sum(self.damage_dice.0, dmg_dice);
            if console::VERBOSE_ACTIONS {
                if die_roll == 20 {
                    println!("Critical hit dealing {} damage!", damage);
                } else {
                    println!("Hit dealing {} damage", damage);
                }
            }
            enemy.take_damage(cmp::max(1,damage));
        } else {
            if console::VERBOSE_ACTIONS { println!("Miss."); }
        }
    } else {
        if console::VERBOSE_ACTIONS { println!("Critical miss!"); }
    }
}

fn long_rest_melee_weapon_attack(ctx:&mut MeleeWeaponAttack) {
    ctx.long_rested = true;
}