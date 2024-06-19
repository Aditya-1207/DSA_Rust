//Problem Statement: Given an array consisting of only 0s, 1s, and 2s.
//Write a program to in-place sort the array without using inbuilt sort functions.
//( Expected: Single pass-O(N) and constant space)


pub fn sort012Array(vec: &mut Vec<usize>) {

    println!("Inside sorting method");
    let mut left:usize = 0;
    let mut mid:usize = 0;
    let mut right = vec.len()-1;

    while mid <= right {
        if vec[mid] == 0 {
            swap(vec,left,mid);
            left += 1;
            mid += 1;
        } else if vec[mid] == 1 {
            mid += 1;
        } else if vec[mid] == 2{
            swap( vec,mid,right);
            right -= 1;
        }
    }

}

pub fn swap(vec: &mut Vec<usize>, mid: usize, right: usize){
    let mut temp :usize = vec[mid];
    vec[mid] = vec[right];
    vec[right] = temp;

}

pub fn main(){
    let mut vector: Vec<usize> = vec![2, 0, 2, 1, 1, 2];
    sort012Array(&mut vector);
    println!("The sorted vector is {:?}",vector);

}