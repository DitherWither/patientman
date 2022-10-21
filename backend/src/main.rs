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
