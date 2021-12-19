use testing::*;

#[test]
fn is_sploosh_splish_ok() {
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
}
