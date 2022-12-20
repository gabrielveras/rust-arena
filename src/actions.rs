use std::cmp;

use crate::characters::{ BaseCharacter };
use crate::rules;

pub trait Action {
    fn update(&mut self, owner:&mut BaseCharacter);
    fn execute(&self, ally:Box<BaseCharacter>, enemy:Box<BaseCharacter>);
}

#[derive(Clone, Debug)]
pub struct MeleeWeaponAttack {
    pub ability_modifier: i32,
    pub damage_dice: (i32,i32),
    pub long_rested: bool,
}

impl MeleeWeaponAttack {

    pub fn new (damage_dice:(i32,i32)) -> Self {
        Self {
            ability_modifier: 0,
            damage_dice: damage_dice,
            long_rested: true
        }
    }

}

impl Action for MeleeWeaponAttack {

    fn update(&mut self, owner:&mut BaseCharacter) {
        let strength_mod = rules::get_ability_modifier(owner.strength);
        let finesse_mod = cmp::max(strength_mod, rules::get_ability_modifier(owner.dexterity));
        //let use_finesse = bool(WeaponProperty.FINESSE in self.weapon_properties)
        let use_finesse = false;
        self.ability_modifier = strength_mod * (!use_finesse as i32) + finesse_mod * (use_finesse as i32);
    }

    fn execute (&self, ally: Box<BaseCharacter>, mut enemy: Box<BaseCharacter>) {
        let die_roll = rules::roll_die(20);
        if die_roll > 1 {
            let attack_roll = die_roll + ally.proficiency + self.ability_modifier;
            if die_roll == 20 || attack_roll >= enemy.armor_class {
                let dmg_dice = self.damage_dice.1; //+ 2*(WeaponProperty.VERSATILE in self.weapon_properties)
                let damage = rules::roll_sum(self.damage_dice.0, dmg_dice, false)
                    + self.ability_modifier;
                    //+ (die_roll >= self.owner.skill_critical_hit) * rules::roll_sum(self.damage_dice.0, dmg_dice);
                enemy.take_damage(cmp::max(1,damage));
            }
        }
    }

}

#[cfg(test)]
mod test {

    #[test]
    fn test_modules() {
        todo!()
    }

}