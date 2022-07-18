use clap::Parser;
use ocipkg::error::*;
use std::{fs, path::PathBuf};

#[derive(Debug, Parser)]
#[clap(version)]
enum Opt {
    /// Pack a directory into an oci-archive tar file
    Pack {
        /// Path of input directory to be packed
        #[clap(parse(from_os_str))]
        input_directory: PathBuf,

        /// Path of output tar archive in oci-archive format
        #[clap(parse(from_os_str))]
        output: PathBuf,

        /// Name of container, use UUID v4 hyphenated if not set.
        #[clap(short = 't', long = "tag")]
        tag: Option<String>,
    },

    /// Load and expand container local cache
    Load {
        /// Input oci-archive
        #[clap(parse(from_os_str))]
        input: PathBuf,
    },

    /// Get and save in local storage
    Get {
        image_name: String,
    },

    /// Push oci-archive to registry
    Push {
        /// Input oci-archive
        #[clap(parse(from_os_str))]
        input: PathBuf,
    },

    /// Get image directory to be used by ocipkg for given container name
    ImageDirectory {
        image_name: String,
    },

    List,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    match Opt::from_args() {
        Opt::Pack {
            input_directory,
            output,
            tag,
        } => {
            let mut output = output;
            output.set_extension("tar");
            if output.exists() {
                panic!("Output already exists: {}", output.display());
            }
            let f = fs::File::create(output)?;
            let mut b = ocipkg::image::Builder::new(f);
            if let Some(name) = tag {
                b.set_name(&ocipkg::ImageName::parse(&name)?);
            }
            b.append_dir_all(&input_directory)?;
            let _output = b.into_inner()?;
        }

        Opt::Load { input } => {
            ocipkg::image::load(&input)?;
        }

        Opt::Get { image_name } => {
            let image_name = ocipkg::ImageName::parse(&image_name)?;
            ocipkg::distribution::get_image(&image_name).await?;
        }

        Opt::Push { input } => {
            ocipkg::distribution::push_image(&input).await?;
        }

        Opt::ImageDirectory { image_name } => {
            let image_name = ocipkg::ImageName::parse(&image_name)?;
            println!("{}", ocipkg::local::image_dir(&image_name)?.display());
        }

        Opt::List => {
            let images = ocipkg::local::get_image_list()?;
            for image in images {
                println!("{}", image);
            }
        }
    }
    Ok(())
}