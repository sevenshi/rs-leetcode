use std::{collections::BTreeMap, usize};


fn two_sum(nums: Vec<i32>,target: i32) -> Vec<i32> {
    let mut map: BTreeMap<i32,usize> = BTreeMap::new();
    for (index ,num ) in nums.iter().enumerate() {
        if map.contains_key(&num){
            return vec![*map.get(num).unwrap() as i32,index as i32]
        }
        map.insert(target- *num, index);
    }
    vec![-1, -1]
}

pub fn run(){
    let result = two_sum(vec![-1,2,3,-2], 0);
    for each in result {
      println!("{}",each)
    }
}