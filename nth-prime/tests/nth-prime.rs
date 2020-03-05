use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}

#[test]
fn test_big_prime2() {
    assert_eq!(np::nth(100_000), 1_299_721);
}

#[test]
fn test_big_prime3() {
    assert_eq!(np::nth(1_000_000), 15_485_867);
}

#[test]
fn test_big_prime4() {
    assert_eq!(np::nth(9_999_999), 179_424_673);
}
