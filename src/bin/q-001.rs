fn main() {
    println!("Hello, world!");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::<i32, usize>::new();

    for (idx, num) in nums.iter().enumerate() {
        let potential = target - num;
        if map.contains_key(&potential) {
            return vec![*map.get(&potential).unwrap() as i32, idx as i32];
        } else {
            map.insert(*num, idx);
        }
    }

    Vec::new()
}
