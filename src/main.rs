
mod characters;
mod equipments;
mod rules;
mod actions;

use actions::Action;
use rand::seq::SliceRandom;

fn main() {
    let mut arena = Arena::new();
    arena.run_simulation()
}

#[derive(Debug)]
struct Arena {
    number_of_years: i32,
    number_of_fighters: i32, 
    fights_per_year: i32,
    max_turns: i32,
    fighters: characters::BaseParty
}

impl Arena {

    pub fn new() -> Self {
        Self {
            number_of_years: 50,
            number_of_fighters: 100,
            fights_per_year: 12,
            max_turns: 20,
            fighters: characters::BaseParty::new()
        }
    }

    pub fn run_simulation(&mut self) {
        // database insert_simulation
        for year in 0..self.number_of_years {
            for _fight in 0..self.fights_per_year {
                self.run_one_year()
            }
            //self.end_year()
        }
        // database insert_many_fighters
    }

    fn run_one_year(&mut self) {
        let size = self.fighters.members.len() as i32;
        for _ in size..self.number_of_fighters {
            let mut fighter = Box::new(characters::BaseCharacter::new_fighter());
            let melee_weapon_attack = Box::new(actions::MeleeWeaponAttack::new((1,6)));
            fighter.add_action(melee_weapon_attack);
            self.fighters.members.push(fighter);
        }
        self.fighters.members.shuffle(&mut rand::thread_rng());
        for _ in (0..self.number_of_fighters).step_by(2) {
        //while self.fighters.members.len() > 0 {
            let mut party_1 = Box::new(characters::BaseParty::new());
            match self.fighters.members.pop() {
                Some(fighter) => party_1.members.push(fighter),
                None => (),
            }
            let mut party_2 = Box::new(characters::BaseParty::new());
            match self.fighters.members.pop() {
                Some(fighter) => party_2.members.push(fighter),
                None => (),
            }
            let winner = self.run_fight(&mut party_1, &mut party_2);
            if winner {
                party_1.add_experience(party_2.get_xp_from_fallen())
            } else {
                party_2.add_experience(party_1.get_xp_from_fallen());
            }
            self.disband_parties(&mut party_1, &mut party_2);
        }
        self.fighters.remove_dead();
        self.fighters.long_rest();
    }

    fn run_fight(&self, party_1:&mut Box<characters::BaseParty>, party_2:&mut Box<characters::BaseParty>) -> bool {
        let mut ellapsed_turns = 0;
        let initiative = rules::flip_coin();
        let mut party_1_is_alive = party_1.any_alive();
        let mut party_2_is_alive = party_2.any_alive();
        while ellapsed_turns < self.max_turns && party_1_is_alive && party_2_is_alive {
            ellapsed_turns += 1;
            if initiative {
                self.act_in_turn(party_1, party_2);
                self.act_in_turn(party_2, party_1);
            } else {
                self.act_in_turn(party_2, party_1);
                self.act_in_turn(party_1, party_2);
            }
            party_1_is_alive = party_1.any_alive();
            party_2_is_alive = party_2.any_alive();
        }
        if party_1_is_alive == party_2_is_alive {
            return rules::flip_coin();
        } else {
            return party_1_is_alive;
        }
    }

    fn act_in_turn(&self, allies:&mut Box<characters::BaseParty>, enemies:&mut Box<characters::BaseParty>) {
        let i_len = allies.members.len();
        let j_len = enemies.members.len();
        for i in 0..i_len {
            for j in 0..j_len {
                if allies.members[i].is_alive() && enemies.members[j].is_alive() {
                    let action = allies.members[i].actions.get(0);
                    match action {
                        Some(a) => a.execute(allies.members[i].clone(), enemies.members[j].clone()),
                        None => (),
                    }
                    continue;
                }
                allies.fall_members();
                enemies.fall_members();
            }
        }
    }

    fn disband_parties(&mut self, party_1:&mut Box<characters::BaseParty>, party_2:&mut Box<characters::BaseParty>) {
        while party_1.members.len() > 0 {
            match party_1.members.pop() {
                Some(x) => self.fighters.members.push(x),
                None => (),
            }
        }
        while party_1.fallen.len() > 0 {
            match party_1.fallen.pop() {
                Some(x) => self.fighters.fallen.push(x),
                None => (),
            }
        }
        while party_2.members.len() > 0 {
            match party_2.members.pop() {
                Some(x) => self.fighters.members.push(x),
                None => (),
            }
        }
        while party_2.fallen.len() > 0 {
            match party_2.fallen.pop() {
                Some(x) => self.fighters.fallen.push(x),
                None => (),
            }
        }
    }

    /*
    fn end_year() {

    }
    */
}
