mod actions;
mod characters;
mod console;
mod equipments;
mod fight_manager;
mod monsters;
mod players;
mod rules;
mod tables;

mod arena;
mod monster_metrics;

const ARENA:bool = false;
const MONSTER_METRICS:bool = true;
const MM_SAMPLE_FIGHT:bool = false && MONSTER_METRICS;
const MM_SINGLE_MATCHUP:bool = true && MONSTER_METRICS;

fn main() {
    if ARENA {
        let mut arena = arena::Arena::test();
        arena.run_simulation()
    } else if MONSTER_METRICS {
        let mut monster_metrics = monster_metrics::MonsterMetric{ matches:20 };
        if MM_SAMPLE_FIGHT {
            monster_metrics.sample_fight();
        } else if MM_SINGLE_MATCHUP {
            monster_metrics.single_matchup(2, 4);
        }
    }
}