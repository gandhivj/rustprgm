pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut mul = vec![];

    for a in factors {
        let mut b = 1;
        loop {
            let val = a * b;
            if val > 0 && val < limit {
                b = b + 1;
                mul.push(val);
            } else {
                break;
            }
        }
    }

    mul.sort();
    mul.dedup();
    mul.iter().sum()
}
