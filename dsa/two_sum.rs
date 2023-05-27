impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(&index) = map.get(num) {
                return vec![index, i as i32];
            } else {
                map.insert(target - num, i as i32);
            }
        }

        Vec::new()
    }
}
