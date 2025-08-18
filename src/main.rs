mod extractors;

use clap::Parser;
use std::io;
use std::path::Path;

/// deal provides one command to handle any archive, so you can finally stop googling "how to unpack X-format".
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// The source URL or local path to the archive
    src: String,

    /// The destination path for extraction
    #[arg()]
    dst: Option<String>,

    /// Automatically create a destination directory based on the archive name
    #[arg(short, long, conflicts_with = "dst")]
    dest_from_src: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();

    let src = cli.src;
    let dst = if cli.dest_from_src {
        generate_dst_from_src(&src)
    } else {
        cli.dst.unwrap_or_else(|| String::from("."))
    };

    validate_destination_path(&dst)?;

    let (pb_option, mut reader) = extractors::get_reader(&src)?;
    let extractor = extractors::create(&src);

    extractor.extract(dst, &mut reader)?;

    if let Some(pb) = pb_option {
        pb.finish_with_message("download and extraction complete");
    } else {
        log::info!("extraction complete");
    }

    Ok(())
}

fn validate_destination_path(dst: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dst);

    match path.symlink_metadata() {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                return Err("destination path cannot be a symbolic link".into());
            }
            if !metadata.is_dir() {
                return Err(format!(
                    "destination path '{}' already exists but is not a directory.",
                    path.display()
                )
                .into());
            }
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

fn generate_dst_from_src(src: &str) -> String {
    let mut all_extensions: Vec<&'static str> = extractors::get_all_extractors()
        .iter()
        .flat_map(|extractor| extractor.get_extensions())
        .collect();

    all_extensions.sort_by_key(|a| std::cmp::Reverse(a.len()));

    let filename = Path::new(src)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(src);

    for ext in all_extensions {
        if filename.ends_with(ext) {
            return filename.strip_suffix(ext).unwrap().to_string();
        }
    }

    filename.to_string()
}
