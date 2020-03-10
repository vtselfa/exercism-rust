pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::<u64>::new();
    for d in (2..=3).chain((1..).flat_map(|x| vec![6 * x - 1, 6 * x + 1])) {
        if n <= 1 {
            break;
        }
        while n % d == 0 {
            factors.push(d);
            n = n / d;
        }
    }
    factors
}
