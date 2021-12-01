pub mod day1;
pub mod day13;

use utils::read_asset as read_global_asset;

fn read_asset(day: i32) -> String {
   read_global_asset(format!("aoc2020/day{}.txt", day))
}
