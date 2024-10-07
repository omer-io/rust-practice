use adder::add;

#[test]
fn it_adds_two() {
    let result = add(2, 5);
    assert_eq!(result, 7);
}