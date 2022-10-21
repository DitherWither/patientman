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

use color_eyre::eyre::Result;
use common::*;

pub struct Database {
    connection: sqlite::Connection,
}

impl Database {
    /// Loads database from file and creates databases if needed
    pub fn init() -> Result<Self> {
        let mut db = Self {
            connection: sqlite::Connection::open("patientman.db")?,
        };

        db.create_table_patients()?;
        Ok(db)
    }

    /// Creates a patients table if it doesn't exist
    fn create_table_patients(&mut self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS patients (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                first_name TEXT NOT NULL,
                last_name TEXT,
                age INTEGER,
                gender TEXT,
                address TEXT
            )",
            [],
        )?;
        Ok(())
    }

    /// Inserts a Patient into the patients table
    /// If the id field of the Patient is 0, A id will be generated automatically
    pub fn insert_into_patients(&mut self, patient: &Patient) -> Result<()> {
        if patient.id == 0 {
            self.connection.execute(
                "INSERT INTO patients (first_name, last_name, age, gender, address)
                    VALUES (?1, ?2, ?3, ?4, ?5)",
                (
                    &patient.first_name,
                    &patient.last_name,
                    &patient.age,
                    &patient.gender.to_string(),
                    &patient.address,
                ),
            )?;
        } else {
            self.connection.execute(
                "INSERT OR REPLACE INTO patients (id, first_name, last_name, age, gender, address)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (
                    &patient.id,
                    &patient.first_name,
                    &patient.last_name,
                    &patient.age,
                    &patient.gender.to_string(),
                    &patient.address,
                ),
            )?;
        }
        Ok(())
    }

    /// Gets all patients in Database and returns them as a Vec.
    pub fn get_all_patients(&self) -> Result<Vec<Patient>> {
        // SQL statement is prepared
        let mut statement = self.connection.prepare("SELECT * FROM patients")?;

        // query_map() is called on the statements to get the rows.
        let patients_rows = statement.query_map([], |row| {
            let gender: String = row.get(4)?;
            let gender: Gender = match gender.to_ascii_uppercase().as_str() {
                "MALE" => Gender::Male,
                "FEMALE" => Gender::Female,
                _ => Gender::Other(gender),
            };

            Ok(Patient {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                age: row.get(3)?,
                gender,
                address: row.get(5)?,
            })
        })?;

        let mut patients = Vec::new();

        // The rows returned by query_map() are MappedRows, so we convert them to a Vec<Patient>
        for patient in patients_rows {
            patients.push(patient?);
        }

        Ok(patients)
    }
}
