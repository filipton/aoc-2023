use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let mut max_map: HashMap<&str, i32> = HashMap::new();
    max_map.insert("red", 12);
    max_map.insert("green", 13);
    max_map.insert("blue", 14);

    let mut id = 0;
    let mut id_sum = 0;
    let mut power_sum = 0;

    'outer: for line in input.lines() {
        id += 1;

        let cubes_str: Vec<&str> = line
            .split(":")
            .last()
            .unwrap()
            .split(",")
            .map(|s| s.split(";").collect::<Vec<_>>())
            .flatten()
            .map(|s| s.trim())
            .collect();

        let cubes_with_colors: Vec<(&str, i32)> = cubes_str
            .iter()
            .map(|s| {
                let s = s.split(" ").collect::<Vec<_>>();
                return (s[1], s[0].parse::<i32>().unwrap());
            })
            .collect();

        let max_red = cubes_with_colors
            .iter()
            .filter(|(color, _)| *color == "red")
            .map(|(_, cubes)| cubes)
            .max()
            .unwrap_or(&0);

        let max_green = cubes_with_colors
            .iter()
            .filter(|(color, _)| *color == "green")
            .map(|(_, cubes)| cubes)
            .max()
            .unwrap_or(&0);

        let max_blue = cubes_with_colors
            .iter()
            .filter(|(color, _)| *color == "blue")
            .map(|(_, cubes)| cubes)
            .max()
            .unwrap_or(&0);

        power_sum += max_red * max_green * max_blue;

        for (color, cubes) in cubes_with_colors {
            if cubes > max_map[color] {
                continue 'outer;
            }
        }

        id_sum += id;
    }

    println!("P1: {}", id_sum);
    println!("P2: {}", power_sum);
}
