use std::collections::HashMap;

pub fn checksums(data: String) -> String {
    let multipliers = data
        .lines()
        .map(check_word)
        .fold((0, 0), |(acc_lh, acc_rh), (lh, rh)| {
            (acc_lh + lh, acc_rh + rh)
        });

    (multipliers.0 * multipliers.1).to_string()
}

pub fn similar_packages(data: String) -> String {
    for line_rh in data.lines() {
        for line_lh in data.lines() {
            let instersection =  line_rh.chars().zip(line_lh.chars()).fold(String::new(), |mut acc, (ch_lh, ch_rh)| {
               if ch_lh == ch_rh {
                   acc.push(ch_lh);
                   return acc;
               } else {
                   return acc;
               }
            });

            if instersection.len()+1 == line_rh.len() {
                return instersection;
            } else {
                ()
            }
        }
    }
    String::new()
}

fn check_word(word: &str) -> (usize, usize) {
    let mut cache: HashMap<char, usize> = HashMap::new();

    for letter in word.chars() {
        *cache.entry(letter).or_insert(0) += 1;
    }

    (
        cache.values().any(|&x| x == 2) as usize,
        cache.values().any(|&x| x == 3) as usize,
    )
}

#[test]
fn check_word_test() {
    assert_eq!((0, 0), check_word("abcdef"));
    assert_eq!((1, 1), check_word("bababc"));
    assert_eq!((1, 0), check_word("abbcde"));
    assert_eq!((0, 1), check_word("abcccd"));
    assert_eq!((1, 0), check_word("aabcdd"));
    assert_eq!((1, 0), check_word("abcdee"));
    assert_eq!((0, 1), check_word("ababab"));
}

#[test]
fn checksums_test() {
    let input = String::from("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab");

    assert_eq!("12", &checksums(input));
}

#[test]
fn similar_packages_test() {
    let input = String::from("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz");

    assert_eq!("fgij", &similar_packages(input));
}