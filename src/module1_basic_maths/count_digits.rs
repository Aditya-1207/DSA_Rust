pub fn count_digits(num:i32) -> usize {

    //1. Brute force
    // let mut count = 0;
    // let mut number = num.abs();
    // while number > 0 {
    //     number /= 10;
    //     count += 1;
    // }
    // return count;

    //2.Slightly Better
    // if num == 0 {
    //     return 1
    // }
    // let mut  count = 1;
    // let mut n = num.abs();
    // while n > 10 {
    //     n /= 10;
    //     count += 1;
    // }
    // count

    //3.Optimised approach : Use log 10 base
    if num == 0 {
        return 1
    }
    //Get the log10 base of the abs|num|
    let count = num.abs().ilog10() as usize;

    //add 1 as log10 (power  of 10) would be 1 less digit and return it
    count+1
}

pub fn main()  {
    //Input
    let number = -2345;
    let count = count_digits(number);
    println!("The number {} has {:?} digits.", number, count);
}