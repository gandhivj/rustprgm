pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut fact = 2;
    let mut result = vec![];

    while fact < n + 1 {
        if n % fact == 0 {
            result.push(fact);
            n /= fact;
        } else {
            fact = fact + 1;
        }
    }

    result
}
