pub fn count_digits(num:i32) -> i32 {

    //1. Brute force
    let mut count = 0;
    let mut number = num.abs();

    while number > 0 {
        number /= 10;
        count += 1;
    }

    return count;

}

pub fn main()  {
    //Input
    let number = 12345;
    let count = count_digits(number);
    println!("The number {} has {:?} digits.", number, count);
}