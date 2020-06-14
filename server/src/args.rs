use clap::{App, Arg};

#[derive(Debug, Clone)]
pub struct Args {
    pub port: String,
    pub master_address: Option<String>,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("server")
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .takes_value(true)
                    .required(true)
                    .help("Port for your server"),
            )
            .arg(
                Arg::with_name("master")
                    .long("master")
                    .short("m")
                    .takes_value(true)
                    .help("Master address to sync to"),
            )
            .get_matches();

        let master_address = match matches.value_of("master") {
            Some(s) => Some(s.to_string()),
            None => None,
        };

        Args {
            port: matches.value_of("port").unwrap().to_string(),
            master_address,
        }
    }
}
