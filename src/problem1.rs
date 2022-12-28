pub fn problem1() {
    let mut vec: Vec<i32> = Vec::new();
    for number in 3..=999 {
        if number % 3 == 0 || number % 5 == 0 {
            vec.push(number)
        }
    }
    let sum: i32 = vec.iter().sum();
    println!(
        "Problem 1: Sum of all the multiples of 3 or 5 below 1000: {}",
        sum
    );
}
