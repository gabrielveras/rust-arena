
use crate::characters::{ BaseCharacter };
use crate::equipments::{ Armor, weapon };
use crate::tables;

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

type LevelUpFunction = fn() -> ();

fn fighter_level_up() {

}

pub fn get_level_up_function() -> LevelUpFunction {
    let lvl_up_fn:LevelUpFunction = fighter_level_up;
    lvl_up_fn
}