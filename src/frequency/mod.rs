use std::collections::HashSet;

pub fn calculate(data: String) -> String {
    data.lines().fold(0, |acc, line| acc + line.parse::<i32>().unwrap()).to_string()
}

pub fn duplication(data: String) -> String {
    // Variable initialization

    let mut frequency: i32 = 0;
    let mut frequencies_cache: HashSet<i32> = HashSet::new();
    frequencies_cache.insert(0);
    let mut converted: Vec<i32> = vec!();

    // First iteration: converting AND checking
    for line in data.lines() {
        let adjust = line.parse::<i32>().unwrap();
        frequency = frequency + adjust;
        converted.push(adjust);

        if frequencies_cache.contains(&frequency) {
            return frequency.to_string();
        } else {
            frequencies_cache.insert(frequency);
        }   
    }

    // All next iterations - over parsed list
    let mut iterator = converted.into_iter().cycle();

    loop {
        frequency = frequency + iterator.next().unwrap();
        if frequencies_cache.contains(&frequency) {
            return frequency.to_string();
        } else {
            frequencies_cache.insert(frequency);
        }
    }
}