use rand::seq::SliceRandom;

use crate::characters::{ BaseParty };
use crate::console;
use crate::fight_manager;
use crate::players;

pub struct Arena {
    number_of_years: i32,
    number_of_fighters: i32, 
    fights_per_year: i32,
    fighters: BaseParty
}

impl Arena {

    /*
    pub fn new() -> Self {
        Self {
            number_of_years: 50,
            number_of_fighters: 100,
            fights_per_year: 12,
            max_turns: 20,
            fighters: BaseParty::new()
        }
    }
    */

    pub fn test() -> Self {
        Self {
            number_of_years: 1,
            number_of_fighters: 100,
            fights_per_year: 12,
            fighters: BaseParty::new()
        }
    }

    pub fn run_simulation(&mut self) {
        self.console_start();
        for year in 0..self.number_of_years {
            if console::VERBOSE_ARENA {
                println!("Year {year}");
            }
            for fight in 0..self.fights_per_year {
                if console::VERBOSE_ARENA {
                    println!("Cycle {fight}");
                }
                self.run_one_cycle()
            }
            //self.end_year()
        }
        self.console_end();
    }

    fn run_one_cycle(&mut self) {
        let size = self.fighters.members.len() as i32;
        for _ in size..self.number_of_fighters {
            self.fighters.members.push_back(players::fighter(1));
        }
        self.fighters.members.make_contiguous().shuffle(&mut rand::thread_rng());
        for _ in (0..self.number_of_fighters).step_by(2) {
            let mut party_1 = BaseParty::new();
            match self.fighters.members.pop_front() {
                Some(fighter) => party_1.members.push_back(fighter),
                None => (),
            }
            let mut party_2 = BaseParty::new();
            match self.fighters.members.pop_front() {
                Some(fighter) => party_2.members.push_back(fighter),
                None => (),
            }
            let winner = fight_manager::run_fight(&mut party_1, &mut party_2);
            if console::VERBOSE_FIGHT {
                if winner {
                    println!("{} won.", party_1);
                } else {
                    println!("{} won.", party_2);
                }
            }
            party_1.add_experience(party_2.get_xp_from_fallen());
            party_2.add_experience(party_1.get_xp_from_fallen());
            self.disband_parties(&mut party_1, &mut party_2);
        }
        self.fighters.remove_dead();
        self.fighters.long_rest();
    }

    pub fn disband_parties(&mut self, party_1:&mut BaseParty, party_2:&mut BaseParty) {
        while party_1.members.len() > 0 {
            match party_1.members.pop_front() {
                Some(x) => self.fighters.members.push_back(x),
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
            match party_2.members.pop_front() {
                Some(x) => self.fighters.members.push_back(x),
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

    fn console_end(&mut self) {
        self.console_fighter_statistics();
        self.console_fighter_data();
        self.console_xp_awards();
    }

    fn console_fighter_data(&mut self) {
        if console::VERBOSE_FIGHTER_DATA {
            println!("Living fighters:");
            for fighter in &mut self.fighters.members {
                println!("{}", fighter);
            }
            if console::VERBOSE_DEAD {
                println!("Dead fighters:");
                for fighter in &mut self.fighters.fallen {
                    println!("{}", fighter);
                }
            }
        }
    }

    fn console_fighter_statistics(&self) {
        
    }

    fn console_start(&self) {
        println!("Settings: {}, numFighters {}, numYears {}, fights/year {}, party size {}, treasure by {}",
            "man-vs-man",
            self.number_of_fighters,
            self.number_of_years,
            self.fights_per_year,
            1,
            "monster"
        );
    }

    fn console_xp_awards(&self) {
        
    }

}