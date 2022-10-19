use color_eyre::eyre::Result;

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
                id INTEGER PRIMARY KEY,
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
}
