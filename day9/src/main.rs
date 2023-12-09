fn main() {
    let input = include_str!("input.txt");
    let numbers_lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;
    let mut sum2 = 0;
    for numbers in numbers_lines {
        let (p1, p2) = get_prediction(numbers);
        sum += p1;
        sum2 += p2;
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}

fn get_prediction(numbers: Vec<i32>) -> (i32, i32) {
    let mut numbers = vec![numbers];
    loop {
        let curr_vec = numbers.last().unwrap();
        if check_if_duplicates(&curr_vec) {
            break;
        }

        let mut tmp_vec = vec![];
        for i in 0..curr_vec.len() - 1 {
            tmp_vec.push(curr_vec[i + 1] - curr_vec[i]);
        }

        numbers.push(tmp_vec);
    }

    for i in (1..numbers.len()).rev() {
        let prediction_1 = numbers[i - 1].last().unwrap() + numbers[i].last().unwrap();
        let prediction_2 = numbers[i - 1].first().unwrap() - numbers[i].first().unwrap();

        numbers[i - 1].push(prediction_1);
        prepend(&mut numbers[i - 1], prediction_2);
    }

    (
        *numbers.first().unwrap().last().unwrap(),
        *numbers.first().unwrap().first().unwrap(),
    )
}

fn check_if_duplicates(numbers: &Vec<i32>) -> bool {
    let mut numbers = numbers.to_vec();
    numbers.dedup();
    return numbers.len() == 1;
}

fn prepend<T>(v: &mut Vec<T>, s: T)
where
    T: Clone,
{
    let mut tmp = vec![s];
    tmp.extend(v.iter().cloned());
    *v = tmp;
}
