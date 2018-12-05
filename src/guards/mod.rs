use chrono::prelude::*;
mod state;
pub use self::state::*;

mod entry;
pub use self::entry::*;

use std::collections::HashMap;

pub fn longest(data: String) -> String {
    let mut ordered = data
        .lines()
        .map(|line| line.parse::<Entry>().unwrap())
        .collect::<Vec<Entry>>();
    ordered.sort();

    let (mut general_state, overall_sleep_count) = create_table(ordered);

    let (guard_id, guard_hours) = overall_sleep_count
        .iter()
        .max_by(|rh, lh| rh.1.cmp(lh.1))
        .unwrap();
    println!("Guard #{} slept {} hours", guard_id, guard_hours);

    let guard_row = general_state.entry(*guard_id).or_insert([0; 60]);
    let max_minutes = guard_row.iter().max().unwrap();
    let best_minute = guard_row.iter().position(|&el| &el == max_minutes).unwrap();

    println!("Best minute is {}", best_minute);

    ((best_minute as u32) * guard_id).to_string()
}

pub fn most_frequent(data: String) -> String {
    let mut ordered = data
        .lines()
        .map(|line| line.parse::<Entry>().unwrap())
        .collect::<Vec<Entry>>();
    ordered.sort();

    let (general_state, _) = create_table(ordered);
    
    let (guard_id, minute, count) = general_state.iter().map(|(&key, value)| {
        let max_minutes = value.iter().max().unwrap();
        let best_minute = value.iter().position(|&el| &el == max_minutes).unwrap();
        (key, best_minute, *max_minutes)
    }).max_by(|lh, rh| lh.2.cmp(&rh.2)).unwrap();

    println!("For guard #{} best minute {} and he slept {}", guard_id, minute, count);

    (guard_id * minute as u32).to_string()
}

fn create_table(entries: Vec<Entry>) -> (HashMap<u32, [u32; 60]>, HashMap<u32, u32>) {
    let mut current_id: u32 = 0;
    let mut begin_sleep: u32 = 0;
    let mut general_state: HashMap<u32, [u32; 60]> = HashMap::new();
    let mut overall_sleep_count: HashMap<u32, u32> = HashMap::new();

    for entry in entries {
        match &entry {
            Entry {
                timestamp: _,
                state: State::Begin(id),
            } => {
                current_id = *id;
            }
            Entry {
                timestamp: time,
                state: State::Asleep,
            } => {
                begin_sleep = time.minute();
            }
            Entry {
                timestamp: time,
                state: State::Wake,
            } => {
                let row = general_state.entry(current_id).or_insert([0; 60]);
                for index in begin_sleep..time.minute() {
                    row[index as usize] += 1;
                }
                overall_sleep_count
                    .entry(current_id)
                    .and_modify(|e| *e += time.minute() - begin_sleep)
                    .or_insert(time.minute() - begin_sleep);
            }
        }
    }

    (general_state, overall_sleep_count)
}
