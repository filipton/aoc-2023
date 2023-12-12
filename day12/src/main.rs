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

        println!("{}", arrg);
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
        if template_idx >= template.len() {
            break;
        }

        match chars[idx] {
            '.' => {
                if hash_count > 0 {
                    if hash_count != template[template_idx] {
                        return tmp_count;
                    }

                    template_idx += 1;
                    hash_count = 0;
                }
            }
            '#' => {
                hash_count += 1;
                if hash_count > template[template_idx] {
                    return tmp_count;
                }
            }
            '?' => {
                // #
                if hash_count + 1 <= template[template_idx] {
                    tmp_count +=
                        get_arragements(idx + 1, chars, hash_count + 1, template_idx, template);
                }

                // .
                if hash_count == 0 {
                    tmp_count += get_arragements(idx + 1, chars, 0, template_idx, template);
                } else if hash_count == template[template_idx] {
                    tmp_count += get_arragements(idx + 1, chars, 0, template_idx + 1, template);
                }

                break;
            }
            _ => unreachable!(),
        }

        idx += 1;
    }

    // END OF RECURSION TREE
    if (template_idx == template.len() - 1 && hash_count == template[template_idx])
        || (template_idx == template.len() && hash_count == 0)
    {
        return 1;
    }

    tmp_count
}
