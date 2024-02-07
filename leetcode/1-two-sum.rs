use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_index_map: HashMap<i32, usize> = HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = num_index_map.get(&complement) {
            return vec![index as i32, i as i32];
        }
        num_index_map.insert(num, i);
    }
    
    vec![]
}

fn main() {
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    println!("{:?}", two_sum(nums1, target1));  // Output: [0, 1]

    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    println!("{:?}", two_sum(nums2, target2));  // Output: [1, 2]

    let nums3 = vec![3, 3];
    let target3 = 6;
    println!("{:?}", two_sum(nums3, target3));  // Output: [0, 1]
}

