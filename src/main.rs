mod extractors;

use std::path::Path;
use std::{env, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let (src, dst) = get_url_from_args()?;

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

fn get_url_from_args() -> Result<(String, String), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("URL was not provided".into());
    }

    let src = args[1].clone();
    let dst: String;

    if args.len() > 2 {
        if args[2] == "-d" {
            // ./deal <src> -d
            dst = generate_dst_from_src(&src);
        } else {
            // ./deal <src> <some_path>
            dst = args[2].clone();
        }
    } else {
        // ./deal <src>
        dst = String::from(".");
    }

    validate_destination_path(&dst)?;

    Ok((src, dst))
}

fn validate_destination_path(dst: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dst);

    match path.symlink_metadata() {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                return Err("destination path cannot be a symbolic link: ".into());
            }
            if metadata.is_file() {
                return Err(format!(
                    "destination path '{}' already exists and is a file.",
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