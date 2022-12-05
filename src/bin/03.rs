use advent_of_code::helpers::letter_to_integer;

pub fn part_one(input: &str) -> Option<u32> {
    // Split the input into lines
    let lines = input.lines();
    // Iterate over the lines
    lines
        .into_iter()
        .map(|l| {
            // Split the lines in half
            let first_half = &l[0..l.len() / 2];
            let second_half = &l[l.len() / 2..];

            // Find duplicate letter
            let mut duplicate_letter = None;
            for (i, c) in first_half.chars().enumerate() {
                if second_half.contains(c) {
                    duplicate_letter = Some(c);
                    break;
                }
            }

            letter_to_integer(duplicate_letter.unwrap_or(char::from(0)))
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    // Split the input into lines
    let lines = input.lines();

    // Group the lines
    // Every set of three lines in your list corresponds to a single group
    let mut groups = Vec::new();
    let mut current_group = Vec::new();
    for (i, l) in lines.into_iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            println!("Pushing group: {:?} {:?}", i, current_group);
            groups.push(current_group);
            current_group = Vec::new();
        }
        current_group.push(l);
    }
    groups.push(current_group);

    println!("{:?}", groups.len());

    // Iterate over the groups and find the duplicate letter
    groups
        .into_iter()
        .map(|g| {
            let mut duplicate_letter = None;
            for (i, c) in g[0].chars().enumerate() {
                if g[1].contains(c) && g[2].contains(c) {
                    duplicate_letter = Some(c);
                    break;
                }
            }

            println!(
                "{:?} {:?}",
                duplicate_letter,
                letter_to_integer(duplicate_letter.unwrap_or(char::from(0)))
            );

            letter_to_integer(duplicate_letter.unwrap_or(char::from(0)))
        })
        .sum::<u32>()
        .into()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
