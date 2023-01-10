use crate::characters::{ BaseParty };
use crate::console;
use crate::fight_manager;
use crate::monsters;
use crate::players;

const FIGHTER_LEVEL:i32 = 1;
const NUMBER_OF_FIGHTERS:i32 = 4;

pub struct MonsterMetric {
    pub matches:i32
}

impl MonsterMetric {

    pub fn sample_fight(&mut self) {
        let mut fighters = BaseParty::new();
        for _ in 0..NUMBER_OF_FIGHTERS {
            fighters.members.push_back(players::fighter(FIGHTER_LEVEL));
        }
        let mut monsters = BaseParty::new();
        monsters.members.push_back(monsters::ghoul());
        let winner = fight_manager::run_fight(&mut fighters, &mut monsters);
        if console::VERBOSE_METRICS {
            if winner {
                println!("{} won.", fighters);
            } else {
                println!("{} won.", monsters);
            }
        }
    }

    pub fn single_matchup(&mut self, fighter_level:i32, number_of_fighters:i32) {
        let mut fighter_wins:i32 = 0;
        for _ in 0..self.matches {
            let ghoul = monsters::ogre();
            let mut monsters = BaseParty::new();
            monsters.members.push_back(ghoul);
            let mut fighters = BaseParty::new();
            for _ in 0..number_of_fighters {
                fighters.members.push_back(players::fighter(fighter_level));
            }
            if console::VERBOSE_METRICS {
                println!("Monsters:");
                for i in 0..monsters.members.len() {
                    println!("{}", monsters.members[i]);
                }
                println!("Fighters:");
                for i in 0..fighters.members.len() {
                    println!("{}", fighters.members[i]);
                }
            }
            let winner = fight_manager::run_fight(&mut fighters, &mut monsters);
            if console::VERBOSE_METRICS {
                if winner {
                    println!("Fighters won.");
                } else {
                    println!("Monsters won.");
                }
            }
            fighter_wins += winner as i32;
        }
        if console::VERBOSE_METRICS {
            println!("Fighters won {} out of {} fights.", fighter_wins, self.matches);
        }
    }

}