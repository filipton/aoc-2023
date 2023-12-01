use std::collections::HashMap;

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("input.txt");
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        let digits = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map_while(|c| c.to_digit(10))
            .collect::<Vec<_>>();

        let num = digits.first().unwrap() * 10 + digits.last().unwrap();
        sum += num;
    }

    println!("PART1: {}", sum);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut digits_map: HashMap<&str, i32> = HashMap::new();
    digits_map.insert("zero", 0);
    digits_map.insert("one", 1);
    digits_map.insert("two", 2);
    digits_map.insert("three", 3);
    digits_map.insert("four", 4);
    digits_map.insert("five", 5);
    digits_map.insert("six", 6);
    digits_map.insert("seven", 7);
    digits_map.insert("eight", 8);
    digits_map.insert("nine", 9);

    let mut sum = 0;
    for line in input.lines() {
        let mut tmp_line = line.to_string();

        let mut first = (100000i32, "", -1);
        let mut last = (-1i32, "", -1);

        for digit in digits_map.iter() {
            let first_index = first_index(&tmp_line, digit.0).map(|i| i as i32);
            let last_index = last_index(&tmp_line, digit.0).map(|i| i as i32);

            if first_index.is_some() && first_index.unwrap() < first.0 {
                first = (first_index.unwrap(), digit.0, digit.1.clone());
            }

            if last_index.is_some() && last_index.unwrap() > last.0 {
                last = (last_index.unwrap(), digit.0, digit.1.clone());
            }
        }

        if !first.1.is_empty() {
            tmp_line = tmp_line.replace(first.1, &first.2.to_string());
        }
        if !last.1.is_empty() {
            tmp_line = tmp_line.replace(last.1, &last.2.to_string());
        }

        println!("{} -> {}", line, tmp_line);

        let digits = tmp_line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map_while(|c| c.to_digit(10))
            .collect::<Vec<_>>();

        let num = digits.first().unwrap() * 10 + digits.last().unwrap();
        sum += num;
    }

    println!("PART2: {}", sum);
}

fn first_index(s: &str, pat: &str) -> Option<usize> {
    s.match_indices(pat).map(|(i, _)| i).next()
}

fn last_index(s: &str, pat: &str) -> Option<usize> {
    s.match_indices(pat).map(|(i, _)| i).last()
}
