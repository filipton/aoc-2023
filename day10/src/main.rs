fn main() {
    let input = include_str!("input.txt");
    let tiles_map = input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_pos = tiles_map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| {
                if *tile == Tile::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut steps = 1;
    let mut tmp_pos = get_first_tile(&tiles_map, start_pos).expect("No first tile found");

    loop {
        if tmp_pos.0 == start_pos.0 && tmp_pos.1 == start_pos.1 {
            break;
        }

        let tile = tiles_map
            .get(tmp_pos.1)
            .and_then(|row| row.get(tmp_pos.0))
            .expect("No tile found");

        let direction = tile.get_direction(&tmp_pos.2).expect("No direction found");
        let direction_pos = direction.get_relative_pos();

        tmp_pos.0 = (tmp_pos.0 as isize + direction_pos.0) as usize;
        tmp_pos.1 = (tmp_pos.1 as isize + direction_pos.1) as usize;
        tmp_pos.2 = direction;

        steps += 1;
    }

    println!("Part 1: {:?}", steps / 2);
    println!("Part 2: {:?}", part2(tiles_map, start_pos));
}

fn part2(mut tiles: Vec<Vec<Tile>>, start_pos: (usize, usize)) -> usize {
    let mut connected_directions = vec![];
    if let Some(tile) = tiles
        .get(start_pos.1.wrapping_sub(1))
        .and_then(|row| row.get(start_pos.0))
    {
        if *tile == Tile::Vertical || *tile == Tile::SouthEast || *tile == Tile::SouthWest {
            connected_directions.push(Direction::North);
        }
    }

    if let Some(tile) = tiles
        .get(start_pos.1 + 1)
        .and_then(|row| row.get(start_pos.0))
    {
        if *tile == Tile::Vertical || *tile == Tile::NorthEast || *tile == Tile::NorthWest {
            connected_directions.push(Direction::South);
        }
    }

    if let Some(tile) = tiles
        .get(start_pos.1)
        .and_then(|row| row.get(start_pos.0 + 1))
    {
        if *tile == Tile::Horizontal || *tile == Tile::NorthWest || *tile == Tile::SouthWest {
            connected_directions.push(Direction::East);
        }
    }

    if let Some(tile) = tiles
        .get(start_pos.1)
        .and_then(|row| row.get(start_pos.0.wrapping_sub(1)))
    {
        if *tile == Tile::Horizontal || *tile == Tile::NorthEast || *tile == Tile::SouthEast {
            connected_directions.push(Direction::West);
        }
    }

    if connected_directions.len() != 2 {
        panic!("Invalid start position");
    }

    tiles[start_pos.1][start_pos.0] =
        Tile::from_directions(&connected_directions[0], &connected_directions[1]);

    let mut insides = 0;
    for y in 0..tiles.len() {
        for x in 0..tiles[y].len() {
            if tiles[y][x] != Tile::Ground {
                continue;
            }

            let mut intersections = 0;
            let mut last_tile = &Tile::Ground;
            for check_x in x + 1..tiles[y].len() {
                let check_tile = &tiles[y][check_x];
                if (last_tile == &Tile::NorthEast && check_tile == &Tile::SouthWest)
                    || (last_tile == &Tile::SouthEast && check_tile == &Tile::NorthWest)
                {
                    last_tile = check_tile;
                    continue;
                }

                if check_tile == &Tile::Vertical
                    || check_tile == &Tile::NorthEast
                    || check_tile == &Tile::NorthWest
                {
                    intersections += 1;
                }

                last_tile = check_tile;
            }

            if intersections % 2 != 0 {
                tiles[y][x] = Tile::GroundInside;
                insides += 1;
            }
        }
    }

    insides
}

fn get_first_tile(
    tiles_map: &Vec<Vec<Tile>>,
    start_pos: (usize, usize),
) -> Option<(usize, usize, Direction)> {
    let north_tile = tiles_map
        .get(start_pos.1.wrapping_sub(1))
        .and_then(|row| row.get(start_pos.0))
        .unwrap_or(&Tile::Ground);

    let south_tile = tiles_map
        .get(start_pos.1 + 1)
        .and_then(|row| row.get(start_pos.0))
        .unwrap_or(&Tile::Ground);

    let east_tile = tiles_map
        .get(start_pos.1)
        .and_then(|row| row.get(start_pos.0 + 1))
        .unwrap_or(&Tile::Ground);

    let west_tile = tiles_map
        .get(start_pos.1)
        .and_then(|row| row.get(start_pos.0.wrapping_sub(1)))
        .unwrap_or(&Tile::Ground);

    if north_tile == &Tile::Vertical
        || north_tile == &Tile::SouthEast
        || north_tile == &Tile::SouthWest
    {
        Some((start_pos.0, start_pos.1 - 1, Direction::North))
    } else if south_tile == &Tile::Vertical
        || south_tile == &Tile::NorthEast
        || south_tile == &Tile::NorthWest
    {
        Some((start_pos.0, start_pos.1 + 1, Direction::South))
    } else if east_tile == &Tile::Horizontal
        || east_tile == &Tile::NorthWest
        || east_tile == &Tile::SouthWest
    {
        Some((start_pos.0 + 1, start_pos.1, Direction::East))
    } else if west_tile == &Tile::Horizontal
        || west_tile == &Tile::NorthEast
        || west_tile == &Tile::SouthEast
    {
        Some((start_pos.0 - 1, start_pos.1, Direction::West))
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Ground, // .
    GroundInside,
    Start,      // S
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthEast,  // F
    SouthWest,  // 7
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Ground,
            'S' => Tile::Start,
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            'F' => Tile::SouthEast,
            '7' => Tile::SouthWest,
            _ => panic!("Unknown tile: {}", c),
        }
    }

    fn from_directions(prev: &Direction, next: &Direction) -> Self {
        match (prev, next) {
            (Direction::North, Direction::South) => Tile::Vertical,
            (Direction::South, Direction::North) => Tile::Vertical,
            (Direction::East, Direction::West) => Tile::Horizontal,
            (Direction::West, Direction::East) => Tile::Horizontal,
            (Direction::South, Direction::East) => Tile::SouthEast,
            (Direction::South, Direction::West) => Tile::SouthWest,
            (Direction::North, Direction::East) => Tile::NorthEast,
            (Direction::North, Direction::West) => Tile::NorthWest,
            _ => panic!("Unknown tile: {:?} {:?}", prev, next),
        }
    }

    fn get_direction(&self, prev: &Direction) -> Option<Direction> {
        match self {
            Tile::Vertical => match prev {
                Direction::North => Some(Direction::North),
                Direction::South => Some(Direction::South),
                _ => None,
            },
            Tile::Horizontal => match prev {
                Direction::East => Some(Direction::East),
                Direction::West => Some(Direction::West),
                _ => None,
            },
            Tile::NorthEast => match prev {
                Direction::South => Some(Direction::East),
                Direction::West => Some(Direction::North),
                _ => None,
            },
            Tile::NorthWest => match prev {
                Direction::South => Some(Direction::West),
                Direction::East => Some(Direction::North),
                _ => None,
            },
            Tile::SouthEast => match prev {
                Direction::North => Some(Direction::East),
                Direction::West => Some(Direction::South),
                _ => None,
            },
            Tile::SouthWest => match prev {
                Direction::North => Some(Direction::West),
                Direction::East => Some(Direction::South),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Direction {
    fn get_relative_pos(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}
