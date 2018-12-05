use regex::Regex;

const FABRIC_SIZE: usize = 1000;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug, Default, Hash)]
struct Claim {
    id: usize,
    offset_top: usize,
    offset_left: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn new(description: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(?P<id>[0-9]+) @ (?P<offset_left>[0-9]+),(?P<offset_top>[0-9]+): (?P<width>[0-9]+)x(?P<height>[0-9]+)").unwrap();
        }
        let data = RE.captures(description).unwrap();
        Claim {
            id: data.name("id").unwrap().as_str().parse().unwrap(),
            offset_left: data.name("offset_left").unwrap().as_str().parse().unwrap(),
            offset_top: data.name("offset_top").unwrap().as_str().parse().unwrap(),
            width: data.name("width").unwrap().as_str().parse().unwrap(),
            height: data.name("height").unwrap().as_str().parse().unwrap(),
        }
    }

    fn as_stripe(&self, fabric_size: usize) -> Vec<usize> {
        let mut stripe = Vec::with_capacity(self.width * self.height);

        for row in self.offset_left..self.offset_left + self.width {
            for col in self.offset_top..self.offset_top + self.height {
                stripe.push(row * fabric_size + col);
            }
        }
        stripe
    }
}

pub fn overlap(data: String) -> String {
    let mut fabric = vec![0; FABRIC_SIZE * FABRIC_SIZE];
    let mut overlap_counter = 0;

    for claim_line in data.lines() {
        for index in Claim::new(claim_line).as_stripe(FABRIC_SIZE) {
            match fabric[index] {
                1 => {
                    fabric[index] += 1;
                    overlap_counter += 1;
                }
                0 => fabric[index] += 1,
                _ => (),
            }
        }
    }
    overlap_counter.to_string()
}

pub fn unoverlaped(data: String) -> String {
    let mut fabric = vec![0; FABRIC_SIZE * FABRIC_SIZE];
    let mut claims: Vec<Claim> = Vec::new();

    for claim_line in data.lines() {
        let claim = Claim::new(claim_line);
        claims.push(claim);

        for index in claim.as_stripe(FABRIC_SIZE) {
            match fabric[index] {
                1 => fabric[index] += 1,
                0 => fabric[index] += 1,
                _ => (),
            }
        }
    }

    for claim in claims {
        if claim.as_stripe(FABRIC_SIZE).iter().any(|index| fabric[*index] == 2) {
            ()
        } else {
            return claim.id.to_string();
        }
    }

    String::from("Unique fabric is not found!")
}

#[test]
fn new_test() {
    assert_eq!(
        Claim {
            id: 1,
            offset_top: 3,
            offset_left: 1,
            width: 4,
            height: 4
        },
        Claim::new("#1 @ 1,3: 4x4")
    );
    assert_eq!(
        Claim {
            id: 2,
            offset_top: 1,
            offset_left: 3,
            width: 4,
            height: 4
        },
        Claim::new("#2 @ 3,1: 4x4")
    );
    assert_eq!(
        Claim {
            id: 3,
            offset_top: 5,
            offset_left: 5,
            width: 2,
            height: 2
        },
        Claim::new("#3 @ 5,5: 2x2")
    );
}

// #1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2
