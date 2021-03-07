use std::{
    io::{self, ErrorKind},
    process::{self, Command, Output},
};

use clap::{load_yaml, App};

fn check_out(output: Result<Output, io::Error>) {}

fn main() {
    let yaml = load_yaml!("setup.yml");
    let app = App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"));
    let config = took::configurate(app).unwrap();
    let db_url = took::db_conn_string(&config).unwrap();
    println!("using database url `{}` to setup the tok database", &db_url);

    let output = Command::new("diesel")
        .arg("setup")
        .arg("--migration-dir")
        .arg("took-models/migrations")
        .env("DATABASE_URL", &db_url)
        .output();
    check_out(output);
    println!("databse migrations were successfully run");
}
