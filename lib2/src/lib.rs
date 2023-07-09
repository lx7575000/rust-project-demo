
pub mod lib2_universe;

use lib2_universe::universe::Universe;

pub fn get_universe() -> Universe {
    Universe::new()
}

pub fn sub(left: i32, right: i32) -> i32 {
    left - right
}
