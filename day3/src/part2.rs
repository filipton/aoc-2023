pub fn part2() {
    let input = include_str!("input.txt");
    let lines_of_chars = input
        .lines()
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut gear_sum = 0;

    for y in 0..lines_of_chars.len() {
        for x in 0..lines_of_chars[y].len() {
            if lines_of_chars[y][x] == '*' {
                let nums = find_adj_numbers(&lines_of_chars, x as isize, y as isize);
                if nums.len() == 2 {
                    gear_sum += nums[0] * nums[1];
                }
            }
        }
    }

    println!("Part 2: {}", gear_sum);
}

#[inline(always)]
fn find_adj_numbers(input: &Vec<Vec<char>>, x: isize, y: isize) -> Vec<u32> {
    let mut tmp_nums: Vec<u32> = Vec::new();

    for xs in (x - 1)..=(x + 1) {
        for ys in (y - 1)..=(y + 1) {
            if (xs == x && ys == y)
                || xs < 0
                || ys < 0
                || ys > input.len() as isize
                || xs > input[ys as usize].len() as isize
            {
                continue;
            }

            let xs = xs as usize;
            let ys = ys as usize;

            if input[ys][xs].is_ascii_digit() {
                let num = inner_find_number(&input, xs, ys);
                if !tmp_nums.contains(&num) {
                    tmp_nums.push(num);
                }
            }
        }
    }

    return tmp_nums;
}

#[inline(always)]
fn inner_find_number(input: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut left_x = None;
    let mut right_x = None;

    for x in (0..=x).rev() {
        if !input[y][x].is_ascii_digit() {
            left_x = Some(x + 1);
            break;
        }
    }

    for x in x..input[y].len() {
        if !input[y][x].is_ascii_digit() {
            right_x = Some(x);
            break;
        }
    }

    let left_x = left_x.unwrap_or(0);
    let right_x = right_x.unwrap_or(input[y].len());

    let num = &input[y][left_x..right_x];
    let num = num
        .iter()
        .fold(0, |acc, &x| acc * 10 + x.to_digit(10).unwrap_or(0) as u32);

    return num;
}
