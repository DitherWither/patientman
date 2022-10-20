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
    /// Converts the Gender enum to a String
    pub fn to_string(&self) -> String {
        match self {
            Self::Male => String::from("MALE"),
            Self::Female => String::from("FEMALE"),
            Self::Other(text) => text.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    /// Unit test for Gender::to_string()
    fn test_gender_to_string() {
        assert_eq!(Gender::Male.to_string(), "MALE");
        assert_eq!(Gender::Female.to_string(), "FEMALE");
        assert_eq!(
            Gender::Other("SomeOtherGender".to_string()).to_string(),
            "SomeOtherGender"
        )
    }
}
