use std::string::ToString;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use quicli::prelude::*;
use structopt::StructOpt;
use itertools::Itertools;
use human_panic::setup_panic;
use structopt::clap::arg_enum;
use toml;

mod api;
mod wikipedia;
mod util;
mod config;

#[derive(StructOpt)]
#[structopt(name="postcode", about="the stupid content tracker")]
enum Command {
    /// Gets information about a given list of postcodes.
    #[structopt(name="get")]
    Get {
        /// A list of postcodes to fetch
        #[structopt()]
        postcodes: Vec<String>,
    },

    /// Gets nearby attractions to a given postcode.
    #[structopt(name="nearby")]
    Nearby {
        /// The postcode to look up
        #[structopt()]
        postcode: String,
    },

    /// Generates a config file in the current directory.
    #[structopt(name="init")]
    Init {
        /// Overwrites previous config, if one exists
        #[structopt(long = "force", short = "f")]
        force: bool,
    }
}

arg_enum! {
    enum Country {
        GB,
        US,
    }
}

/// Fetches postcode data from postcodes.io, as well a few other things.
#[derive(StructOpt)]
struct Cli {
    /// The ISO3166 country code to search for
    #[structopt(long = "country", short = "c", default_value = "GB")]
    country: Country,

    #[structopt(subcommand)]
    command: Command,

    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}

#[tokio::main]
async fn main() -> util::Result<()> {
    setup_panic!();

    let path = Path::new("./.postcodeconfig");
    let config = if path.exists() {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        toml::from_slice(buf.as_slice())?
    }  else {
        config::Config::default()
    };

    let args = Cli::from_args();
    let p_client: Box<dyn api::postcode::PostcodeClient> = match args.country {
        Country::GB => Box::new(config.postcodes_io),
        Country::US => Box::new(config.target_lock),
    };
    let w_client = wikipedia::WikipediaClient{};

    match args.command {
        Command::Get{postcodes} => {
            let postcodes = p_client.get_postcodes(postcodes).await?;
            println!("{}", postcodes.iter().map(ToString::to_string).join("\n"));
        },
        Command::Nearby{postcode} => {
            let postcode = p_client.get_postcode(postcode).await?;
            let nearby = w_client.get_nearby(postcode).await?;
            println!("{}", nearby.iter().map(ToString::to_string).join("\n"));
        },
        Command::Init{force} => {
            let path = Path::new("./.postcodeconfig");
            if path.exists() && !force {
                println!("Warning: there is already a config file in this folder.");
                println!("If you wish to overwrite it, please supply the -f flag.");
                return Ok(())
            }

            let config = config::Config::default();
            let serialized = toml::to_vec(&config)?;
            let mut out = File::create(path)?;
            out.write_all(serialized.as_slice())?;
        },
    };

    Ok(())
}