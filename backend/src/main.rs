mod db;

use color_eyre::eyre::Result;
use db::Database;

fn main() -> Result<()> {
    // Installs the color_eyre panic and error report hanlers
    color_eyre::install()?;

    let db = Database::init()?;

    Ok(())
}
