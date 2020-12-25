const MOD_NUM: isize = 20201227;
fn main() {
    let card_public_key = 16915772;
    let door_public_key = 18447943;
    let card_loop = find_loop_size(card_public_key, 7);
    let door_loop = find_loop_size(door_public_key, 7);
    let card_encryption_key = get_encryption_key(card_public_key, door_loop);
    let door_encryption_key = get_encryption_key(door_public_key, card_loop);

    assert_eq!(card_encryption_key, door_encryption_key);
    println!("part1 solution: {:?}", door_encryption_key);
}

fn find_loop_size(target: isize, subject_number: isize) -> isize {
    let mut curr_val = 1;
    (1..)
        .find(|_| {
            curr_val = transform(curr_val, subject_number);
            curr_val == target
        })
        .unwrap()
}

fn transform(val: isize, subject_number: isize) -> isize {
    (val * subject_number) % MOD_NUM
}

fn get_encryption_key(subject_number: isize, loop_size: isize) -> isize {
    (0..loop_size).fold(1, |acc, _| transform(acc, subject_number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let card_public_key = 5764801;
        let door_public_key = 17807724;

        let card_loop_size = find_loop_size(card_public_key, 7);
        let door_loop_size = find_loop_size(door_public_key, 7);
        assert_eq!(card_loop_size, 8);
        assert_eq!(door_loop_size, 11);

        let card_encryption_key = get_encryption_key(door_public_key, card_loop_size);
        let door_encryption_key = get_encryption_key(card_public_key, door_loop_size);
        assert_eq!(card_encryption_key, 14897079);
        assert_eq!(door_encryption_key, 14897079);
    }
}
