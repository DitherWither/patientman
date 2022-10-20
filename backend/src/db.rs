use color_eyre::eyre::Result;
use common::Patient;

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
}
