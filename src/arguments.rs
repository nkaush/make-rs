use std::path::PathBuf;
use clap::Parser;

/// A Rust implementation of the GNU make utility.
#[derive(Parser, Debug)]
#[clap(name = "make-rs")]
#[clap(version, author, about, long_about = None)]
pub struct Arguments {
    /// Read FILE as a Makefile.
    #[arg(short, long, visible_aliases = ["makefile"], default_value = "Makefile")]
    pub file: PathBuf,

    /// Change to DIRECTORY before doing anything.
    #[arg(short = 'C', long)]
    pub directory: Option<PathBuf>,

    /// Allow JOBS jobs at once; infinite jobs with no arg.
    #[arg(short, long, default_value_t = 1)]
    pub jobs: u8,

    /// Don't echo commands.
    #[arg(short, long, visible_aliases = ["quiet"])]
    pub silent: bool,

    /// Unconditionally make all targets.
    #[arg(short = 'B', long)]
    pub always_make: bool,

    /// Don't actually run any commands; just print them.
    #[arg(short = 'n', long, visible_aliases = ["just-print, recon"])]
    pub dry_run: bool,

    /// A list of rules to make
    pub rules: Vec<String>
}

pub struct MakeConfiguration {
    pub silent: bool,
    pub always_make: bool,
    pub dry_run: bool
}

impl Arguments {
    pub fn get_configuration(&self) -> MakeConfiguration {
        MakeConfiguration { 
            silent: self.silent, 
            always_make: self.always_make, 
            dry_run: self.dry_run
        }
    }
}