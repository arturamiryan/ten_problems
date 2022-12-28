pub fn problem2() {
    let vec = fib(4_000_000);
    let sum_of_even: i32 = vec.iter().filter(|x| *x % 2 == 0).sum();
    println!(
        "Problem 2: Sum of even number in fibonacci sequence that lower 4M: {}",
        sum_of_even
    );
}

fn fib(n: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    fib_iter(1, 0, n, &mut vec);
    return vec;
}

fn fib_iter(a: i32, b: i32, count: i32, vec: &mut Vec<i32>) -> i32 {
    vec.push(b);
    if count < a {
        b
    } else {
        fib_iter(a + b, a, count - 1, vec)
    }
}
