use std::collections::HashSet;

pub fn gcd(a: u32, b: u32) -> u32 {
    if a == b {
        return a;
    }

    if a > b {
        return gcd(a - b, b);
    }

    gcd(a, b - a)
}

pub fn lcm(a: u32, b: u32) -> u32 {
    (a * b) / gcd(a, b)
}

pub fn sum_aritm_prog(n:u32) -> u32 {
    ((n.pow(2) + n) / 2)
}

pub fn sum_aritm_prog_with_lim(m:u32, limit:u32) -> u32 {
    let limit = limit - 1;
    let n = (limit - limit % m) / m; 
    let r = sum_aritm_prog(n) * m;
    println!("m:{} n:{} l:{} r:{}", m, n, limit, r);
    r
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let factors:Vec<_> = factors.iter().filter(|x| **x != 0).collect();
    if factors.is_empty() || limit == 0 {
        return 0;
    }

    let mut result:u32 = 0;
    for m in factors.iter() {
        result += sum_aritm_prog_with_lim(**m, limit);
    }

    let mut lcms:Vec<u32> = vec![];
    for (i, f1) in factors[.. factors.len() - 1].iter().enumerate() {
        for f2 in factors[i + 1 ..].iter() {
            let multiple = lcm(**f1, **f2);
            lcms.push(multiple);
            println!("f1:{} f2:{} lcm:{}", *f1, *f2, multiple);
            result -= sum_aritm_prog_with_lim(lcm(**f1, **f2), limit);
        }
    }

    let mut done = HashSet::<u32>::new();
    if lcms.len() > 2 {
        for (i, f1) in lcms[.. lcms.len() - 1].iter().enumerate() {
            for f2 in lcms[i + 1 ..].iter() {
                let multiple = lcm(*f1, *f2);
                if done.contains(&multiple) {
                    continue;
                }
                done.insert(multiple);
                println!("f1':{} f2':{} lcm':{}", f1, f2, lcm(*f1, *f2));
                result += sum_aritm_prog_with_lim(lcm(*f1, *f2), limit);
            }
        }
    }

    result
}
