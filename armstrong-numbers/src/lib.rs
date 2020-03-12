pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut v = Vec::<u32>::new();
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }

    let mut sum = 0;
    for n in v.iter() {
        sum += n.pow(v.len() as u32); 
    }

    sum == num
}
