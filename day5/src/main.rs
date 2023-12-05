#[derive(Debug, Clone)]
struct MapRange {
    dst: usize,
    src: usize,
    len: usize,

    src_range: std::ops::Range<usize>,
    dst_range: std::ops::Range<usize>,
}

fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines().filter(|x| !x.is_empty()).collect::<Vec<_>>();

    let seeds_p1: Vec<_> = lines[0]
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<usize>().expect("Invalid input"))
        .collect();

    let seeds_p2: Vec<_> = lines[0]
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<usize>().expect("Invalid input"))
        .collect();
    let seeds_p2 = seeds_p2[..]
        .chunks(2)
        .map(|x| (x[0]..x[0] + x[1]))
        .collect::<Vec<_>>();

    let mut map: Vec<Vec<MapRange>> = Vec::new();

    // map parsing
    for l in lines[1..].iter() {
        if l.contains("map") {
            map.push(Vec::new());
            continue;
        }

        let parts: Vec<_> = l
            .split(' ')
            .map(|x| x.parse::<usize>().expect("Invalid input"))
            .collect();

        map.last_mut().unwrap().push(MapRange {
            dst: parts[0],
            src: parts[1],
            len: parts[2],
            src_range: parts[1]..parts[1] + parts[2],
            dst_range: parts[0]..parts[0] + parts[2],
        });
    }

    let mut output_p1 = Vec::new();
    for seed in seeds_p1 {
        let mut curr = seed;

        for m in &map {
            let map_obj = m.iter().find(|x| x.src_range.contains(&curr));
            if let Some(map_obj) = map_obj {
                curr = map_obj.dst + (curr - map_obj.src);
            }
        }

        output_p1.push(curr);
    }
    println!("Part 1: {:?}", output_p1.iter().min().unwrap());

    let mut last_map = map.last().unwrap().to_vec();
    last_map.sort_by(|a, b| a.dst.cmp(&b.dst));

    for o in last_map {
        for dst_seed in o.dst..o.dst + o.len {
            let mut curr = dst_seed;

            for m in map.iter().skip(1).rev() {
                let map_obj = m.iter().find(|x| x.dst_range.contains(&curr));
                if let Some(map_obj) = map_obj {
                    curr = map_obj.src + (curr - map_obj.dst);
                }
            }

            if seeds_p2.iter().any(|x| x.contains(&curr)) {
                println!("Part 2: {}", dst_seed);
                return;
            }
        }
    }
}
