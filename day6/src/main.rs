fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<_>>();
    let times = lines[0]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let dsts = lines[1]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    if times.len() != dsts.len() {
        panic!("times and dsts must be the same length");
    }

    let times_dsts_map = times.iter().zip(dsts.iter()).collect::<Vec<_>>();
    let mut wins = Vec::new();

    for (time, dst) in times_dsts_map {
        wins.push(calc_win_distances(time, dst));
    }
    println!("Part 1: {:?}", wins.iter().fold(1, |acc, x| acc * x));

    // Part 2
    let time = lines[0]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse::<i64>();

    let dst = lines[1]
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse::<i64>();

    println!(
        "Part 2: {}",
        calc_win_distances(&time.unwrap(), &dst.unwrap())
    );
}

#[inline(always)]
fn calc_win_distances(time: &i64, dst: &i64) -> usize {
    let mut left = 0;
    let mut right = *time;
    for hold_time in 1..*time {
        let traveled_distance = hold_time * (time - hold_time);

        if traveled_distance > *dst {
            left = hold_time;
            break;
        }
    }

    for hold_time in (1..*time).rev() {
        let traveled_distance = hold_time * (time - hold_time);

        if traveled_distance > *dst {
            right = hold_time;
            break;
        }
    }

    (left..=right).count()
}
