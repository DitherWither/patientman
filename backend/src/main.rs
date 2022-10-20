mod db;

use color_eyre::eyre::Result;
use common::*;
use db::Database;

fn main() -> Result<()> {
    // Installs the color_eyre panic and error report hanlers
    color_eyre::install()?;

    let mut db = Database::init()?;
    let patient = Patient {
        id: 1,
        first_name: "Turrdhan".to_string(),
        last_name: "Chatil".to_string(),
        age: 15,
        gender: Gender::Male,
        address: "fr, who knows".to_string(),
    };

    db.insert_into_patients(&patient)?;

    Ok(())
}
