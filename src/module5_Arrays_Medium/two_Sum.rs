//Problem Statement: Given an array of integers arr[] and an integer target. Then
//1st : Return YES if there exist two numbers such that their sum is equal to the target. Otherwise, return NO.
//2nd : Return indices of the two numbers such that their sum is equal to the target. Otherwise, we will return {-1, -1}.

use std::collections::HashMap;

pub fn two_sum(vec: Vec<i32>, target: i32) -> &'static str {
    let mut map: HashMap<i32,i32> = HashMap::new();
    let mut index = 0;

    for value in vec.iter() {
        if map.contains_key(&(target - value)) {
                return "Yes";
        } else {
            map.insert(*value, index);
            index += 1;
        }
    }
   return "No";
}


pub fn main()  {
    //Input
    let vector = vec![3, 6, 5, 8, 11];
    let target = 12;

    let contains_sum : &str = two_sum(vector, target);
    println!("Does Array contains the target sum ? {}",contains_sum);

}