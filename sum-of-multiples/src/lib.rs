
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result: u32 = 0;
    for i in 1..limit {
        for f in factors {
            if *f == 0 {
                break;
            }
            if i % f == 0 {
                result += i;
                break;
            }
        }
    }
    result
}
