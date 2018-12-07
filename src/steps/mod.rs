use std::collections::{HashMap, HashSet};

pub fn sequence(data: String) -> String {
    let mut state = get_initial_state(&data);
    // println!("Initial state {:?}", state);

    let mut result = String::new();

    loop {
        let current_action = next_available_action(&state).unwrap();
        state.remove(&current_action);
        result.push(current_action);

        if state.is_empty() {
            return result;
        }

        for (_action, requirements) in &mut state {
            requirements.remove(&current_action);
        }
    }
}

pub fn parallel(data: String, base_time: usize, workers_number: usize) -> String {
    let mut state = get_initial_state(&data);
    // println!("Initial state {:?}", state);

    let mut plot: Vec<Vec<char>> = Vec::with_capacity(workers_number);

    for _i in 0..workers_number {
        plot.push(vec!['*'; state.len() * 60]);
    }

    let mut work_ends: HashMap<char, usize> = HashMap::new();

    'outer: for i in 0..&state.len() * 60 - 1 {
        // println!("Tick #{}", i);
        // println!("Finished jobs list: {:?}", &work_ends);
        // End of the plot
        if state.len() == 0 {
            if plot.iter().all(|worker| worker[i] == '*') {
                // Plotting output
                // Strategy 1 - row
                // for worker in &plot {
                //     println!("{}", String::from_iter(worker.iter().take(i+1)));
                // }
                // Strategy 2 - column
                for i in 0..i + 1 {
                    let mut output = String::new();
                    plot.iter().for_each(|worker| output.push(worker[i]));
                    println!("{}", output);
                }
                return i.to_string();
            } else {
                continue;
            }
        }

        // Removing finished jobs
        for (work, end_tick) in &work_ends {
            if end_tick == &i {
                // println!("Job {} is finished. Removing...", &work);
                for (_action, requirements) in &mut state {
                    requirements.remove(&work);
                }
            }
        }

        // Still have something to put

        // Get next work:
        'inner: loop {
            // Looping untill still have available jobs
            match next_available_action(&state) {
                Some(action) => {
                    // println!("New available action: {}!", &action);
                    for worker in &mut plot {
                        if worker[i] == '*' {
                            // Plot work onto plot
                            for j in i..i + (base_time + 1 + action as usize - 'A' as usize) {
                                worker[j] = action;
                            }
                            // Set works end
                            work_ends.insert(
                                action,
                                i + (base_time + 1 + action as usize - 'A' as usize),
                            );
                            // Remove work from state
                            state.remove(&action);
                            continue 'inner;
                        }
                    }
                    // Didn't find available worker
                    continue 'outer;
                }
                // No more tasks
                None => continue 'outer,
            }
        }
    }

    String::new()
}

/// Finds next available action, and returns it's name. If no available actions - returns None.
fn next_available_action(state: &HashMap<char, HashSet<char>>) -> Option<char> {
    let mut available_actions: Vec<char> = Vec::new();

    for (&action, requirements) in state {
        if requirements.is_empty() {
            available_actions.push(action)
        }
    }
    if available_actions.len() == 0 {
        None
    } else {
        available_actions.sort();
        Some(available_actions[0])
    }
}

fn get_initial_state(data: &String) -> HashMap<char, HashSet<char>> {
    let mut state: HashMap<char, HashSet<char>> = HashMap::new();

    for statement in data.lines() {
        let root: char = statement.as_bytes()[5] as char;
        let child: char = statement.as_bytes()[36] as char;
        {
            let entry = state.entry(child).or_insert(HashSet::new());
            entry.insert(root);
        }
        // Ensure, that work that has no roots is in the list
        state.entry(root).or_insert(HashSet::new());
    }
    state
}

#[test]
fn sequence_test() {
    let input = String::from(
        "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.",
    );

    assert_eq!(String::from("CABDFE"), sequence(input));
}

#[test]
fn parallel_test() {
    let input = String::from(
        "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.",
    );

    assert_eq!(String::from("15"), parallel(input, 0, 2));
}
