use rand::{random, thread_rng, Rng};
use std::cmp;

fn flip_coin() -> bool {
    random()
}

fn roll_die(size:i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(0..size)
}

fn get_ability_modifier(ability_score:i32) -> i32 {
    (((ability_score-10) as f32) / 2.0).floor() as i32
}

pub fn get_max_hit_points(hit_dice:i32, rolled_hit_points:i32, hit_die_size:i32, constitution:i32) -> i32 {
    let max_hp = hit_die_size + rolled_hit_points + (hit_dice * get_ability_modifier(constitution));
    cmp::max(1, max_hp)
}

#[cfg(test)]
mod tests {
    use super::get_ability_modifier;
    use super::get_max_hit_points;

    #[test]
    fn test_negative_ability_mod() {
        assert_eq!(-2, get_ability_modifier(7));
    }

    #[test]
    fn test_positive_ability_mod() {
        assert_eq!(2, get_ability_modifier(15));
    }

    #[test]
    fn test_big_hit_points() {
        let max_hp = get_max_hit_points(5, 22, 8, 15);
        assert_eq!(40, max_hp);
    }

    #[test]
    fn test_small_hit_points() {
        let max_hp = get_max_hit_points(1, 0, 4, 3);
        assert_eq!(1, max_hp);
    }

}