pub fn part1() {
    let input = include_str!("input.txt");
    let lines_of_chars = input
        .lines()
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut tmp_sum = 0;
    for y in 0..lines_of_chars.len() {
        let mut tmp_nums: Vec<u8> = Vec::new();
        let mut was_adjacent = false;

        for x in 0..lines_of_chars[y].len() {
            if lines_of_chars[y][x].is_ascii_digit() {
                tmp_nums.push(lines_of_chars[y][x].to_digit(10).unwrap() as u8);
                was_adjacent =
                    check_adjacent(&lines_of_chars.clone(), x as isize, y as isize) || was_adjacent;
            } else {
                if was_adjacent {
                    let num = tmp_nums.iter().fold(0, |acc, &x| acc * 10 + x as u32);
                    tmp_sum += num;

                    was_adjacent = false;
                }

                tmp_nums.clear();
            }
        }

        if was_adjacent {
            let num = tmp_nums.iter().fold(0, |acc, &x| acc * 10 + x as u32);
            tmp_sum += num;
        }
    }

    println!("Part 1: {}", tmp_sum);
}

#[inline(always)]
fn check_adjacent(input: &Vec<Vec<char>>, x: isize, y: isize) -> bool {
    for xs in (x - 1)..=(x + 1) {
        for ys in (y - 1)..=(y + 1) {
            if xs == x && ys == y {
                continue;
            }

            if check_adjacent_spot(&input, xs, ys) {
                return true;
            }
        }
    }

    return false;
}

#[inline(always)]
fn check_adjacent_spot(input: &Vec<Vec<char>>, x: isize, y: isize) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    let x = x as usize;
    let y = y as usize;

    if (0..input.len()).contains(&y) {
        if (0..input[y].len()).contains(&x) {
            let c = input[y][x];

            if !c.is_ascii_digit() && c != '.' {
                return true;
            }
        }
    }

    return false;
}
