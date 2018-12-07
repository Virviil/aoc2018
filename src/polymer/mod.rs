trait PolymerItem {
    fn opposite(rh: &Self, lh: &Self) -> bool;
}

impl PolymerItem for char {
    fn opposite(rh: &char, lh: &char) -> bool {
        if rh.is_lowercase() {
            &rh.to_ascii_uppercase() == lh
        } else {
            &rh.to_ascii_lowercase() == lh
        }
    }
}

pub fn triggered(data: String) -> String {
    triggered_length(data.chars()).to_string()
}

fn triggered_length<T>(polymer: T) -> usize
where
    T: Iterator,
    T::Item: PolymerItem + Clone,
{
    let mut result: Vec<T::Item> = polymer.collect();
    loop {
        match trigger_cycle(&result) {
            Some(new_result) => result = new_result,
            None => break,
        }
    }
    result.len()
}

fn trigger_cycle<T>(polymer: &[T]) -> Option<Vec<T>>
where
    T: PolymerItem + Clone,
{
    let mut changed = false;
    let mut new_polymer: Vec<T> = Vec::with_capacity(polymer.len());
    let mut polymer_iterator = polymer.iter().peekable();

    let mut skip_next = false;
    loop {
        if skip_next {
            polymer_iterator.next();
            skip_next = false;
        } else {
        };
        match polymer_iterator.next() {
            Some(current_element) => match polymer_iterator.peek() {
                Some(next_element) => {
                    if PolymerItem::opposite(current_element, next_element) {
                        skip_next = true;
                        changed = true;
                    } else {
                        new_polymer.push(current_element.clone());
                    }
                }
                None => new_polymer.push(current_element.clone()),
            },
            None => break,
        }
    }
    if changed {
        Some(new_polymer)
    } else {
        None
    }
}

pub fn best_strand(data: String) -> String {
    let mut results: Vec<usize> = Vec::new();

    for letter in (0..26).map(|x| (x + 'a' as u8) as char) {
        results
            .push(triggered_length(data.chars().filter(|ch| {
                ch != &letter && ch != &letter.to_ascii_uppercase()
            })));
    }
    results.iter().min().unwrap().to_string()
}

#[test]
fn opposite_test() {
    assert_eq!(true, PolymerItem::opposite(&'A', &'a'));
    assert_eq!(true, PolymerItem::opposite(&'a', &'A'));
    assert_eq!(false, PolymerItem::opposite(&'A', &'b'));
    assert_eq!(false, PolymerItem::opposite(&'a', &'D'));
}

#[test]
fn triggered_test() {
    assert_eq!(
        String::from("dabCBAcaDA"),
        triggered(String::from("dabAcCaCBAcCcaDA"))
    );
}
