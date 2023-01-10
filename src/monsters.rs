
use crate::actions::{ MeleeWeaponAttack };
use crate::characters;
use crate::equipments::{ Armor };
use crate::equipments::weapon::{ WeaponProperty };

// CR 1
pub fn ghoul() -> Box<characters::BaseCharacter> {
    let data = characters::CharacterData {
        name:String::from("Ghoul"),
        strength:13,
        dexterity:15,
        constitution:10,
    };
    let armor = Armor::natural(0);
    let mut ghoul = characters::BaseCharacter::new_monster(5, 8, data, armor);
    ghoul.add_action(MeleeWeaponAttack::new(String::from("Bite"), (2,6), WeaponProperty::NONE));
    ghoul.add_action(MeleeWeaponAttack::new(String::from("Claws"), (2,4), WeaponProperty::NONE));
    ghoul.update();
    ghoul.complete_heal();
    Box::new(ghoul)
}

// CR 2
pub fn ogre() -> Box<characters::BaseCharacter> {
    let data = characters::CharacterData {
        name:String::from("Ogre"),
        strength:19,
        dexterity:8,
        constitution:16,
    };
    let armor = Armor::hide();
    let mut ogre = characters::BaseCharacter::new_monster(7, 10, data, armor);
    ogre.add_action(MeleeWeaponAttack::new(String::from("Greatclub"), (2,8), WeaponProperty::NONE));
    ogre.add_action(MeleeWeaponAttack::new(String::from("Javelin"), (2,6), WeaponProperty::NONE));
    ogre.update();
    ogre.complete_heal();
    Box::new(ogre)
}
