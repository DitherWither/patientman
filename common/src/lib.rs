
#[derive(Debug)]
pub struct Patient {
    id: u32,
    first_name: String,
    last_name: String,
    age: i32,
    gender: Gender,
    address: String,
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other(String),
}
