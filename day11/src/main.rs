fn main() {
    let input = include_str!("input.txt");
    let map = expand_universe(input);
    let galaxies = get_galaxies(&map, 2);
    let pairs = get_pairs(&galaxies);

    let mut sum = 0;
    for ((x1, y1), (x2, y2)) in pairs {
        sum += (x1 - x2).abs() + (y1 - y2).abs();
    }
    println!("Part 1: {}", sum);

    let galaxies = get_galaxies(&map, 1000000);
    let pairs = get_pairs(&galaxies);
    let mut sum = 0;
    for ((x1, y1), (x2, y2)) in pairs {
        sum += (x1 - x2).abs() + (y1 - y2).abs();
    }

    println!("Part 2: {}", sum);
}

fn get_galaxies(map: &Vec<Vec<char>>, empty_space_size: i128) -> Vec<(i128, i128)> {
    let mut galaxies: Vec<(i128, i128)> = vec![];

    let mut real_y = 0;
    for y in 0..map.len() {
        if map[y][0] == '@' {
            real_y += empty_space_size;
            continue;
        }

        let mut real_x = 0;
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                real_x += empty_space_size;
                continue;
            }

            if map[y][x] == '#' {
                galaxies.push((real_x, real_y));
            }

            real_x += 1;
        }

        real_y += 1;
    }

    galaxies
}

fn expand_universe(input: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<String> = vec![];

    for line in input.lines() {
        let line = line.trim();

        if !line.chars().any(|c| c != '.') {
            // if line is only dots
            lines.push(line.replace(".", "@"));
        } else {
            lines.push(line.to_string());
        }
    }

    let mut map = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for x in 0..map[0].len() {
        let mut tmp_dots = 0;

        for y in 0..map.len() {
            if map[y][x] == '.' || map[y][x] == '@' {
                tmp_dots += 1;
            }
        }

        if tmp_dots == map.len() {
            replace_column(&mut map, x, '@');
        }
    }

    map
}

fn replace_column<T>(lines: &mut Vec<Vec<T>>, column: usize, c: T)
where
    T: Copy,
{
    for line in lines {
        line[column] = c;
    }
}

fn get_pairs<T>(input: &[T]) -> Vec<(&T, &T)> {
    let mut pairs = vec![];

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            pairs.push((&input[i], &input[j]));
        }
    }

    pairs
}
