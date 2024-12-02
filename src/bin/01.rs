advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];
    
    for line in lines
    {
        let values: Vec<&str> = line.split("   ").collect();
        if values.len() == 2
        {
            left_list.push(values.get(0)?.parse::<u32>().unwrap());
            right_list.push(values.get(1)?.parse::<u32>().unwrap());
        }
    }
    left_list.sort();
    right_list.sort();
    let mut value: u32 = 0;
    for (x,y) in left_list.iter().zip(right_list.iter())
    {
        value += x.abs_diff(*y);
    }
    Some(value)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
