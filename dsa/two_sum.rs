use std::collections::HashMap;


fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(&index) = map.get(num) {
            return vec![index, i as i32];
        } else {
            map.insert(target - num, i as i32);
        }
    }

    Vec::new()
}

fn main() {
    // Problem taken from leetcode
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let result = two_sum(nums, target);

    println!("Indices are: {:?}", result);
}

