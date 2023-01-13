// Just brute-forcing it

fn func(n: i64) -> i64 {
    let mut n = n;
    let mut count = 0;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        count += 1;
    }
    count
}

pub fn problem14() {
    let mut res = 0;
    let mut count = 0;
    for i in 3..999999 {
        let d = func(i);
        if d > count {
            res = i;
            count = d;
        }
    }

    println!(
        "Problem 14: Starting number, under one million, produces the longest chain: {}",
        res
    );
}
