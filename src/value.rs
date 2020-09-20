//The value of each card
#[derive(Eq, PartialEq, Hash)]
pub struct Value(u8);
impl Value {
    pub fn new(number: u8) -> Value {
        if number < 1 || number > 13 {
            panic!("Card value must be between 1 and 13");
        }
        Value(number)
    }
}
