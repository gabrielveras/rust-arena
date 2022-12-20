use rand;
use rand::Rng;
use std::cmp;

pub const XP_PER_CHAR_LEVEL:i32 = 200;

pub fn flip_coin() -> bool {
    rand::random()
}

pub fn roll_die(size:i32) -> i32 {
    rand::thread_rng().gen_range(1..size)
}

pub fn roll_dice(number:i32, size:i32) -> Vec<i32> {
    (0..number).map(|_| rand::thread_rng().gen_range(1..size) ).collect()
}

pub fn roll_sum(number:i32, size:i32, drop_lowest:bool) -> i32 {
    let roll: Vec<i32> = roll_dice(number, size);
    let sum:i32 = roll.iter().sum();
    match roll.iter().min() {
        Some(min) => sum - min * (drop_lowest as i32),
        None => sum,
    }
}

pub fn get_ability_modifier(ability_score:i32) -> i32 {
    (((ability_score-10) as f32) / 2.0).floor() as i32
}

pub fn get_max_hit_points(hit_dice:i32, rolled_hit_points:i32, hit_die_size:i32, constitution:i32) -> i32 {
    let max_hp = hit_die_size + rolled_hit_points + (hit_dice * get_ability_modifier(constitution));
    cmp::max(1, max_hp)
}

pub fn get_proficiency(hit_dice:i32) -> i32 {
    ((((hit_dice-1) as f32) / 4.0).floor() as i32) + 1
}

#[cfg(test)]
mod tests {
    use super::roll_sum;
    use super::get_ability_modifier;
    use super::get_max_hit_points;

    #[test]
    fn test_roll_sum() {
        let roll:i32 = roll_sum(4, 6, true);
        let check:bool = (3 <= roll) && (roll <= 18);
        assert_eq!(true, check);
    }

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