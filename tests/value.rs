use poker::value::Value;


#[test]
fn create_valid_value() {
    for value in 1..14 {
        Value::new(value);
    }
}

#[test]
#[should_panic]
fn wrong_value() {
    Value::new(0);
}
