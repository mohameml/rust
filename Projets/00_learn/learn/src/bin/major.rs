use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut freq: HashMap<i32, f64> = HashMap::new();
    let n = nums.len();

    for num in nums {
        let val = freq.entry(num).or_insert(0);
        *val += 1 / n;
    }

    println!("freq :");
    println!("{:#?}", freq);

    let mut res = Vec::new();

    for (k, v) in freq {
        if v > 1 / 3 as f64 {
            res.push(k)
        }
    }

    res
}

fn main() {
    let nums = vec![3, 2, 3];

    let res = majority_element(nums);

    println!("{:?}", res);
}
