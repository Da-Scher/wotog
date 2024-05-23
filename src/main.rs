use clap::{Parser, Subcommand};

#[path="wotog.rs"]
mod wotog;

#[derive(Parser)]
#[command(version,about,long_about = None)]
struct Cli{
    #[arg(short, long, value_name="FILE")]
    config: Option<std::path::PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: Option<u8>,

    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands{
    Add {
        paths: Vec<std::path::PathBuf>,
    },
    Init {
        #[arg(short('n'), long("no-shell"))]
        no_shell: Option<bool>,
        #[arg(short = 'c', long = "config")]
        config: Option<String>,
    },
    Config {
        //TODO: do we want global?
        //global: bool,
        change: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let verbosity: u8 = match &cli.debug {
        None => 0,
        Some(0) => 0,
        Some(1) => 1,
        Some(2) => 2,
        Some(_) => 2,
    };
    match &cli.commands {
        Some(Commands::Add { paths }) => {
            println!("Add command");
        },
        Some(Commands::Init { no_shell, config }) => {
            wotog::wotog_init(verbosity, config.clone());
        },
        Some(Commands::Config { change }) => {
            println!("Config command");
        },
        None => todo!(),
    }
}
