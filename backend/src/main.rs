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

use db::Database;
use std::sync::Mutex;

// Maybe there is a better way to do this but this should work for now
static DB: Mutex<Option<Database>> = Mutex::new(None);

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, World"
}


#[rocket::launch]
fn rocket() -> _ {
    {
        *DB.lock().unwrap() = Some(Database::init().unwrap());
    }

    color_eyre::install().unwrap();
    rocket::build()
        .mount("/", rocket::routes![index])
}
