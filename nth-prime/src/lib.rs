// Trivial solution
pub fn is_prime(n: u32) -> bool {
    // There is no point in checking numbers greater than the square root
    let sr = (n as f64).sqrt() as u32;

    for i in 2..sr + 1 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

// A bit smarter solution
pub fn is_prime_mem(n: u32, primes: &Vec<u32>) -> bool {
    // There is no point in checking numbers greater than the square root
    let sr = (n as f64).sqrt() as u32;

    // Only consider the already computed prime numbers as potential dividers
    for i in primes.iter() {
        if *i > sr {
            break;
        }
        if n % *i == 0 {
            return false;
        }
    }

    true
}

pub fn nth(n: u32) -> u32 {
    let mut v: Vec<u32> = Vec::new();
    let n = n as usize;

    // It's the only even prime, we will only check odd numbers
    v.push(2);

    let mut candidate : u32 = 3;

    while v.len() <= n {
       if is_prime_mem(candidate, &v) { 
            v.push(candidate);
       }
       candidate += 2;
    }
    
    v[n]
}
