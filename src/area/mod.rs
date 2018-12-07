pub fn largest(data: String) -> String {
    let mut x_min: i32 = std::i32::MAX;
    let mut y_min: i32 = std::i32::MAX;
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;

    let mut points: Vec<(usize, i32, i32)> = Vec::with_capacity(50);

    for (index, coords) in data.lines().enumerate() {
        let mut pair = coords.split(", ");
        let x: i32 = pair.next().unwrap().parse().unwrap();
        let y: i32 = pair.next().unwrap().parse().unwrap();

        if x < x_min {
            x_min = x;
        }
        if y < y_min {
            y_min = y;
        }
        if x > x_max {
            x_max = x;
        }
        if y > y_max {
            y_max = y;
        }

        points.push((index, x, y));
    }

    // println!(
    //     "X: min={} max={}, Y: min={} max={}",
    //     x_min, x_max, y_min, y_max
    // );
    // println!("Points are: {:?}", &points);

    let mut results: Vec<i32> = vec![0; points.len()];
    let mut bad_indexes: Vec<usize> = Vec::with_capacity(50);

    for y in y_min - 1..y_max + 1 {
        for x in x_min - 1..x_max + 1 {
            let mut min_distance: (usize, i32) = (std::usize::MAX, std::i32::MAX);
            let mut undefined: bool = false;

            // Minifying distance
            for point in &points {
                let current_distance: i32 = manhattan_distance(&(point.1, point.2), &(x, y));
                if current_distance == min_distance.1 {
                    undefined = true;
                }
                if current_distance < min_distance.1 {
                    min_distance = (point.0, current_distance);
                    undefined = false;
                }
            }

            if !undefined {
                results[min_distance.0] += 1;

                if x == x_min || x == x_max || y == y_min || y == y_max {
                    bad_indexes.push(min_distance.0)
                }
                // print!("{}", ('@' as u8 + min_distance.0 as u8) as char)
            } else {
                // print!("*");
            }
        }
        // print!("\n")
    }

    let min_area = results
        .iter()
        .enumerate()
        .filter(|(index, _value)| !bad_indexes.contains(index))
        .map(|(_, value)| value)
        .max()
        .unwrap();
    min_area.to_string()
}

pub fn region(data: String) -> String {
    let mut x_min: i32 = std::i32::MAX;
    let mut y_min: i32 = std::i32::MAX;
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;

    let mut points: Vec<(usize, i32, i32)> = Vec::with_capacity(50);

    for (index, coords) in data.lines().enumerate() {
        let mut pair = coords.split(", ");
        let x: i32 = pair.next().unwrap().parse().unwrap();
        let y: i32 = pair.next().unwrap().parse().unwrap();

        if x < x_min {
            x_min = x;
        }
        if y < y_min {
            y_min = y;
        }
        if x > x_max {
            x_max = x;
        }
        if y > y_max {
            y_max = y;
        }

        points.push((index, x, y));
    }

    // println!(
    //     "X: min={} max={}, Y: min={} max={}",
    //     x_min, x_max, y_min, y_max
    // );
    // println!("Points are: {:?}", &points);

    let mut area_size: i32 = 0;

    for y in y_min - 1..y_max + 1 {
        for x in x_min - 1..x_max + 1 {
            let mut total_distance: i32 = 0;

            // Calculating total distance
            for point in &points {
                let current_distance: i32 = manhattan_distance(&(point.1, point.2), &(x, y));
                total_distance += current_distance;
            }

            if total_distance < 10000 {
                area_size += 1;
                // print!("#");
            } else {
                // print!(".");
            }
        }
        // print!("\n")
    }

    area_size.to_string()
}

fn manhattan_distance(rh: &(i32, i32), lh: &(i32, i32)) -> i32 {
    (rh.0 - lh.0).abs() + (rh.1 - lh.1).abs()
}
