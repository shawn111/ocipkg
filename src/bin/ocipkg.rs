use log::info;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ocipkg", about = "OCI Registry for binary distribution")]
enum Opt {
    /// Pack a directory into an oci-archive tar file
    Pack {
        /// Input directory
        #[structopt(parse(from_os_str))]
        input_directory: PathBuf,

        /// Output oci archive
        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },

    /// Load and expand container local cache
    Load {
        /// Input oci-archive
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },

    /// Initialize local storage
    Initialize {},
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    match Opt::from_args() {
        Opt::Pack {
            input_directory,
            output,
        } => {
            ocipkg::compose::compose(&input_directory, &output)?;
        }

        Opt::Load { input } => {
            dbg!(input);
            todo!()
        }

        Opt::Initialize {} => {
            let dir = ocipkg::config::data_dir()?;
            if !dir.is_dir() {
                info!("Create directory: {}", dir.display());
                fs::create_dir_all(&dir)?;
            }
        }
    }
    Ok(())
}
