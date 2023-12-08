use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<_>>();

    let instructions = lines[0]
        .chars()
        .map(|i| {
            if i == 'L' {
                0
            } else if i == 'R' {
                1
            } else {
                panic!("Invalid input")
            }
        })
        .collect::<Vec<_>>();

    let mut nodes: HashMap<&str, [&str; 2]> = HashMap::new();
    for line in lines[2..].iter() {
        let splitted = line.split('=').collect::<Vec<_>>();
        let name = splitted[0].trim();

        let coords = splitted[1]
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(',')
            .map(|i| i.trim())
            .collect::<Vec<_>>();

        nodes.insert(name, coords.try_into().unwrap());
    }

    let mut steps = 0;
    let mut current_node = "AAA";

    'outer: loop {
        for instruction in instructions.iter() {
            let next_node = nodes.get(current_node).unwrap()[*instruction];
            steps += 1;
            current_node = next_node;

            if current_node == "ZZZ" {
                break 'outer;
            }
        }
    }

    println!("Part 1: {:?}", steps);

    let mut all_steps = Vec::new();
    let mut current_nodes = nodes
        .keys()
        .filter(|i| i.ends_with("A"))
        .map(|i| *i)
        .collect::<Vec<_>>();

    for current_node in current_nodes.iter_mut() {
        let mut steps = 0;
        let mut current_node = *current_node;

        'outer: loop {
            for instruction in instructions.iter() {
                let next_node = nodes.get(current_node).unwrap()[*instruction];
                steps += 1;
                current_node = next_node;

                if current_node.ends_with("Z") {
                    break 'outer;
                }
            }
        }

        all_steps.push(steps);
    }

    println!("Part 2: {:?}", lcm(all_steps));
}

fn lcm(numbers: Vec<usize>) -> usize {
    let mut result = numbers[0];
    for i in 1..numbers.len() {
        result = result * numbers[i] / gcd(result, numbers[i]);
    }
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
