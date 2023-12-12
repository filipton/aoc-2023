fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines();

    let mut arragements = 0;
    for l in lines {
        let parts = l.split(" ").collect::<Vec<&str>>();
        let template = parts[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let question_marks = parts[0].matches("?").count();
        let combinations = get_combinations(question_marks)
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| if *y == 0 { '.' } else { '#' })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        for c in combinations {
            let mut ci = 0;
            let mut hash_count = 0;
            let mut c_template: Vec<usize> = vec![];

            for i in 0..parts[0].len() {
                let curr = match parts[0].chars().nth(i).unwrap() {
                    '?' => {
                        ci += 1;
                        c[ci - 1]
                    }
                    '.' => '.',
                    '#' => '#',
                    _ => panic!("Unknown char"),
                };

                if curr == '#' {
                    hash_count += 1;
                } else if curr == '.' {
                    if hash_count > 0 {
                        c_template.push(hash_count);
                        hash_count = 0;
                    }
                }
            }

            if hash_count > 0 {
                c_template.push(hash_count);
            }

            if c_template == template {
                arragements += 1;
            }
        }
    }

    println!("Arragements: {}", arragements);
}

// get_combinations should return vector of vector of usize's, so for n = 2 it should return [[0,0], [0,1], [1,0], [1,1]]
// its 2^n combinations
fn get_combinations(n: usize) -> Vec<Vec<usize>> {
    let mut combinations = Vec::new();
    for i in 0..2usize.pow(n as u32) {
        let mut combination = Vec::new();
        for j in 0..n {
            combination.push((i >> j) & 1);
        }
        combinations.push(combination);
    }
    combinations
}
