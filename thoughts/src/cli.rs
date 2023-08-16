use clap::{Parser, Subcommand};
use crate::config::Config;
use crate::prompt;
use crate::get_path;

#[derive(Parser, Debug)]
#[clap(author=Config::AUTHOR, version=Config::VERSION_STRING, about=Config::ABOUT)]
struct CliArgs {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about="To start today's random thought session")]
    Today,
    #[clap(about="To initialise a new database of thoughts")]
    Init,
    #[clap(about="To export your thoughts as a mark-down document")]
    Export,
}

impl Commands {
    pub fn execute(&self) {
        match self {
            Commands::Today => prompt::session(get_path()),
            Commands::Init => prompt::init(get_path()),
            Commands::Export => self.test(),
        }
    }

    fn test(&self) {
        println!("Testing testing...");
        println!("Hello World!");
        println!("It's working lmao");
    }
}

static mut COUNT: u8 = 0; // Counts the amount of times cli is run (cannot be more than once!)

pub fn run() {
    // Safety checks
    unsafe { if COUNT > 0 { panic!("Error: cli cannot be run more than once during program lifetime!") } }
    unsafe { COUNT += 1; }
    let cli: CliArgs = CliArgs::parse();
    cli.command.execute();
}