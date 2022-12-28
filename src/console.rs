const ALLOW_VERBOSE:bool = true;
pub const VERBOSE_ARENA:bool = ALLOW_VERBOSE && false;
pub const VERBOSE_FIGHT:bool = ALLOW_VERBOSE && false;
pub const VERBOSE_ACTIONS:bool = VERBOSE_FIGHT && false;
pub const VERBOSE_FIGHTER_DATA:bool = ALLOW_VERBOSE && false;
pub const VERBOSE_DEAD:bool = VERBOSE_FIGHTER_DATA && false;