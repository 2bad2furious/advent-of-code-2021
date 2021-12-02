pub mod day1;
pub mod day2;

use utils::read_asset as read_global_asset;

fn read_asset(day: i32) -> String {
   read_global_asset(format!("aoc2021/day{}.txt", day))
}
