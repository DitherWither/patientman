#[derive(Debug)]
pub struct Patient {
    /// Set to 0 if id is unknown, when inserting into database,
    /// It will be generated automatically
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub gender: Gender,
    pub address: String,
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other(String),
}
impl Gender {
    pub fn to_string(&self) -> String {
        match self {
            Self::Male => String::from("MALE"),
            Self::Female => String::from("FEMALE"),
            Self::Other(text) => text.to_string(),
        }
    }
}
