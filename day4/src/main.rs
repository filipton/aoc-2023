fn main() {
    let input = include_str!("input.txt");
    let cards = input
        .lines()
        .map(|l| l.split(":").collect::<Vec<_>>()[1])
        .collect::<Vec<_>>();

    let cards_numbers = cards
        .iter()
        .map(|c| {
            let splitted = c.split("|").collect::<Vec<_>>();

            let winning = splitted[0]
                .trim()
                .split(" ")
                .filter(|n| !n.is_empty())
                .map_while(|n| n.parse::<u32>().ok())
                .collect::<Vec<_>>();

            let your = splitted[1]
                .trim()
                .split(" ")
                .filter(|n| !n.is_empty())
                .map_while(|n| n.parse::<u32>().ok())
                .collect::<Vec<_>>();

            let common = winning
                .into_iter()
                .filter(|n| your.contains(n))
                .collect::<Vec<_>>();

            common
        })
        .collect::<Vec<_>>();

    let mut s_nums = vec![1; cards.len()];

    let mut i = 0;
    let mut total = 0;
    for common in cards_numbers {
        for n in 0..common.len() {
            s_nums[n + i + 1] += s_nums[i];
        }

        if common.len() > 0 {
            let pow = 2u32.pow(common.len() as u32 - 1);
            total += pow;
        }

        i += 1;
    }

    println!("Part 1: {}", total);
    println!("Part 2: {}", s_nums.iter().sum::<u32>());
}
