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

        let chars = parts[0].chars().collect::<Vec<char>>();
        let arrg = get_arragements(0, &chars, 0, 0, &template);
        arragements += arrg;

        /*
        let arg_brt = get_brt(parts[0], template.clone());
        if arrg != arg_brt {
            println!("{} {} - {}/{}", parts[0], parts[1], arg_brt, arrg);
        }
        */
    }

    println!("Part 1: {}", arragements);
}

fn get_arragements(
    mut idx: usize,
    chars: &Vec<char>,
    mut hash_count: usize,
    mut template_idx: usize,
    template: &Vec<usize>,
) -> usize {
    let mut tmp_count = 0;

    while idx < chars.len() {
        match chars[idx] {
            '.' => {
                if hash_count > 0 {
                    if hash_count != *template.get(template_idx).unwrap_or(&0) {
                        return 0;
                    }

                    template_idx += 1;
                    hash_count = 0;
                }
            }
            '#' => {
                hash_count += 1;
                if hash_count > *template.get(template_idx).unwrap_or(&0) {
                    return 0;
                }
            }
            '?' => {
                // #
                if hash_count + 1 <= *template.get(template_idx).unwrap_or(&0) {
                    tmp_count +=
                        get_arragements(idx + 1, chars, hash_count + 1, template_idx, template);
                }

                // .
                if hash_count == 0 {
                    tmp_count += get_arragements(idx + 1, chars, 0, template_idx, template);
                } else if hash_count == *template.get(template_idx).unwrap_or(&0) {
                    tmp_count += get_arragements(idx + 1, chars, 0, template_idx + 1, template);
                }

                return tmp_count;
            }
            _ => unreachable!(),
        }

        idx += 1;
    }

    // END OF RECURSION TREE
    if idx >= chars.len() {
        if (template_idx == template.len() - 1 && hash_count == template[template_idx])
            || (template_idx == template.len() && hash_count == 0)
        {
            //println!("END REC {} {} {} {}", idx, template_idx, hash_count, template.len());
            return 1;
        }
    }

    tmp_count
}

fn get_brt(input: &str, template: Vec<usize>) -> usize {
    let mut arg_brt = 0;
    let question_marks = input.matches("?").count();
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

        for i in 0..input.len() {
            let curr = match input.chars().nth(i).unwrap() {
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
            arg_brt += 1;
        }
    }

    arg_brt
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
