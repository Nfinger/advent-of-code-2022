pub fn part_one(input: &str) -> Option<u32> {
    let mut calories = Vec::new();
    let mut current_cals = 0;
    input.lines().for_each(|l| {
        let cal = l.parse::<u32>().unwrap_or(0);
        if (cal > 0) {
            current_cals += cal;
        } else {
            calories.push(current_cals);
            current_cals = 0;
        }
    });

    calories.sort();

    calories.pop()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories = Vec::new();
    let mut current_cals = 0;
    input.lines().for_each(|l| {
        let cal = l.parse::<u32>().unwrap_or(0);
        if (cal > 0) {
            current_cals += cal;
        } else {
            calories.push(current_cals);
            current_cals = 0;
        }
    });

    calories.sort();

    let top_three_cals = calories.iter().rev().take(3).sum::<u32>();
    Some(top_three_cals)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
