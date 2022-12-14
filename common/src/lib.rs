// Copyright (C) 2022  Vardhan Patil
// This file is part of patientman.
//
// patientman is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// patientman is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with patientman.
// If not, see <https://www.gnu.org/licenses/>.

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

impl std::fmt::Display for Gender {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::Male => "MALE",
                Self::Female => "FEMALE",
                Self::Other(text) => text,
            }
        )
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
