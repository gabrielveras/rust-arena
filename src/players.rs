use bitflags::bitflags;

use crate::characters::{ BaseCharacter };
use crate::equipments::{ Armor, weapon };
use crate::tables;

bitflags! {
    pub struct FightingStyle:i32 {
        const NONE               = 0b00000000;
        const ARCHERY            = 0b00000001;
        const DEFENSE            = 0b00000010;
        const DUELING            = 0b00000100;
        const GREAT_WEAPON_FIGHT = 0b00001000;
        const PROTECTION         = 0b00010000;
        const TWO_WEAPON_FIGHT   = 0b00100000;
    }
}

pub enum Class {
    None, Fighter
}

pub fn fighter(level:i32) -> Box<BaseCharacter> {
    let name = tables::random_name();
    let armor = Armor::chain_mail();
    let mut fighter = BaseCharacter::new_player(name, Class::Fighter, 10, armor);
    fighter.add_action(weapon::longsword());
    while fighter.hit_dice < level {
        fighter.add_level();
    }
    fighter.update();
    fighter.complete_heal();
    Box::new(fighter)
}

type LevelUpFunction = fn(&mut BaseCharacter) -> ();

fn fighter_level_up(ctx:&mut BaseCharacter) {
    ctx.fighting_style.set(FightingStyle::DEFENSE, true);
}

pub fn get_level_up_function() -> LevelUpFunction {
    let lvl_up_fn:LevelUpFunction = fighter_level_up;
    lvl_up_fn
}