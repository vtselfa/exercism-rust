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

pub fn nth(n: u32) -> u32 {
    let mut v: Vec<u32> = Vec::new();
    let n = n as usize;
    v.push(2);

    let mut candidate : u32 = 3;

    while v.len() <= n {
       if is_prime(candidate) { 
            v.push(candidate);
       }
       candidate += 2;
    }
    
    println!("{:?}", v);
    v[n]
}
