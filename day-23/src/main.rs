fn main() {
    let input = 963275481;
    println!("part1 solution: {}", move_cups(input, 100));
    println!("part2 solution: {}", move_cups2(input, 10_000_000));
}

fn shuffle(input: usize, num_repeats: usize, num_of_items: usize) -> Vec<usize> {
    let mut successors = vec![0; num_of_items + 1];
    let mut value = input;
    let mut curr = 0;
    let mut prev = 0;
    while value > 0 {
        curr = value % 10;
        successors[curr] = prev;
        value /= 10;
        prev = curr;
    }
    let (last_num_pos, _) = successors
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_i, &v)| v == 0)
        .unwrap();
    successors[0] = curr;
    if num_of_items > 10 {
        successors[last_num_pos] = 10;
        successors
            .iter_mut()
            .enumerate()
            .skip(10)
            .for_each(|(i, v)| *v = i + 1);
        successors[num_of_items] = curr;
    } else {
        successors[last_num_pos] = curr;
    }

    for _i in 0..num_repeats {
        curr = swap_and_get_next(&mut successors, curr, num_of_items);
    }

    successors
}

fn swap_and_get_next(successors: &mut Vec<usize>, curr: usize, num_of_items: usize) -> usize {
    let item1 = successors[curr];
    let item2 = successors[item1];
    let item3 = successors[item2];
    let picked_up = [item1, item2, item3];
    let mut target_value = if curr <= 1 { num_of_items } else { curr - 1 };
    while picked_up.contains(&target_value) {
        target_value -= 1;
        if target_value < 1 {
            target_value = num_of_items;
        }
    }
    successors.swap(curr, target_value);
    successors.swap(curr, item3);
    successors[curr]
}

fn move_cups(input: usize, n: usize) -> usize {
    let successors = shuffle(input, n, 9);
    let mut curr = successors[1];
    let mut res = 0;
    while curr != 1 {
        res = res * 10 + curr;
        curr = successors[curr];
    }
    res
}

fn move_cups2(input: usize, n: usize) -> usize {
    let successors = shuffle(input, n, 1_000_000);
    let first = successors[1];
    let second = successors[first];
    first * second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(move_cups(389125467, 10), 92658374);
        assert_eq!(move_cups(389125467, 100), 67384529);
        assert_eq!(move_cups2(389125467, 10_000_000), 149245887792);
    }
}
