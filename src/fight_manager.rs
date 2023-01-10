use crate::characters::{ BaseParty };
use crate::rules;
use crate::console;

const MAX_TURNS:i32 = 20;

pub fn run_fight(party_1:&mut BaseParty, party_2:&mut BaseParty) -> bool {
    let mut ellapsed_turns = 0;
    if console::VERBOSE_FIGHT {
        println!("{} vs. {}", party_1, party_2);
    }
    let initiative = rules::flip_coin();
    let mut party_1_is_alive = party_1.any_alive();
    let mut party_2_is_alive = party_2.any_alive();
    while ellapsed_turns < MAX_TURNS && party_1_is_alive && party_2_is_alive {
        ellapsed_turns += 1;
        if initiative {
            act_in_turn(party_1, party_2);
            act_in_turn(party_2, party_1);
        } else {
            act_in_turn(party_2, party_1);
            act_in_turn(party_1, party_2);
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

pub fn act_in_turn(allies:&mut BaseParty, enemies:&mut BaseParty) {
    let i_len = allies.members.len();
    let j_len = enemies.members.len();
    for i in 0..i_len {
        if console::VERBOSE_FIGHT {
            println!("{} ({}) turn.", allies.members[i].data.name, i);
        }
        for j in 0..j_len {
            if allies.members[i].is_alive() && enemies.members[j].is_alive() {
                if console::VERBOSE_FIGHT {
                    println!("{} ({}) targets {} ({}).",
                        allies.members[i].data.name, i,
                        enemies.members[j].data.name, j);
                }
                let action = allies.members[i].actions.get(0);
                match action {
                    Some(a) => a.execute(&allies.members[i], &mut enemies.members[j]),
                    None => (),
                }
                break;
            }
        }
    }
    allies.fall_members();
    enemies.fall_members();
}