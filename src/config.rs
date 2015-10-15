use std::error::Error;
use std::io;
use std::io::Read;
use std::fs::File;
use toml::{Parser, Table};
use postgres;

const CONFIG_TOML: &'static str = "./chien.toml";

pub struct Config {
    psql_conn: postgres::Connection,
}

fn parse_db_params(parsed: Option<Table>) -> (Option<postgres::UserInfo>, Option<String>) {
    // if there's a table
    if let Some(table) = parsed {
        // if there's a postgres section
        if let Some(section) = table.get("postgres").and_then(|x| x.as_table()) {
            // grab maybe the DB
            let database = section.get("db")
                .and_then(|db| db.as_str().map(|s| s.to_owned()));

            // grab maybe the user and convert it to a string
            let user_params = section.get("user")
                .and_then(|user|
                    user.as_str().to_owned().map(|u|
                        // take this string and shove it into a UserInfo
                        postgres::UserInfo {
                            user: u.to_owned(),
                            // alongside maybe the password
                            password: section.get("pass")
                                .and_then(|p| p.as_str().map(|s| s.to_owned()))
                        }
                    )
                );

            (user_params, database)
        } else {
            (None, None)
        }
    } else {
        (None, None)
    }
}

impl Config {
    pub fn new() -> Result<Config, Box<Error>> {
        let mut config_file = try!(File::open(CONFIG_TOML));
        let mut config_s = String::new();
        try!(config_file.read_to_string(&mut config_s));

        // grab the data from a file
        let parsed = Parser::new(&config_s).parse();

        // load the database parameters
        let (user_params, database) = parse_db_params(parsed);
        let conn_params = postgres::ConnectParams {
            target: postgres::ConnectTarget::Tcp("localhost".to_owned()),
            port: None,
            user: user_params,
            database: database,
            options: Vec::new(),
        };

        // connect to the database
        let conn = try!(postgres::Connection::connect(
            conn_params,
            &postgres::SslMode::None,
        ));

        // return the config
        Ok(Config {
            psql_conn: conn,
        })
    }
}
