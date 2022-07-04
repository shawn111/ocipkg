use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ocipkg", about = "OCI Registry for binary distribution")]
enum Opt {
    /// Pack a directory into an oci-archive tar file
    Pack {
        /// Path of input directory to be packed
        #[structopt(parse(from_os_str))]
        input_directory: PathBuf,

        /// Path of output tar archive in oci-archive format
        #[structopt(parse(from_os_str))]
        output: PathBuf,

        /// Name of container, use UUID v4 hyphenated if not set.
        #[structopt(short = "t", long = "tag")]
        tag: Option<String>,
    },

    /// Load and expand container local cache
    Load {
        /// Input oci-archive
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },

    /// Get and save in local storage
    Get { image_name: String },

    /// Get image directory to be used by ocipkg for given container name
    ImageDirectory { image_name: String },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    match Opt::from_args() {
        Opt::Pack {
            input_directory,
            output,
            tag,
        } => {
            let mut output = output;
            output.set_extension("tar");
            if output.exists() {
                anyhow::bail!("Output already exists");
            }
            let f = fs::File::create(output)?;
            let mut b = ocipkg::image::Builder::new(f);
            if let Some(name) = tag {
                b.set_name(&name)?;
            }
            let cfg = oci_spec::image::ImageConfigurationBuilder::default().build()?;
            b.append_config(cfg)?;
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

        Opt::ImageDirectory { image_name } => {
            let image_name = ocipkg::ImageName::parse(&image_name)?;
            println!("{}", ocipkg::config::image_dir(&image_name)?.display());
        }
    }
    Ok(())
}