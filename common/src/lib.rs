pub struct Patients {
    id: u32,
    first_name: String,
    last_name: String,
    age: i32,
    gender: Gender,
    address: String,
}

pub enum Gender {
    Male,
    Female,
    Other(String),
}
