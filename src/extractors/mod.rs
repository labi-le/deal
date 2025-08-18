mod bz2;
mod gz;
mod rar;
mod raw;
mod sevenz;
mod tar;
mod tarbz2;
mod targz;
mod tarxz;
mod xz;
mod zip;

pub use bz2::Bz2Extractor;
pub use gz::GzExtractor;
pub use rar::RarExtractor;
pub use raw::RawFileExtractor;
pub use sevenz::SevenZExtractor;
pub use tar::TarExtractor;
pub use tarbz2::TarBz2Extractor;
pub use targz::TarGzExtractor;
pub use tarxz::TarXzExtractor;
pub use xz::XzExtractor;
pub use zip::ZipExtractor;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub trait Extractor {
    fn extract(
        &self,
        path: String,
        reader: &mut dyn Read,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn get_extensions(&self) -> Vec<&'static str>;
}

pub fn create(url: &str) -> Box<dyn Extractor> {
    let extractors = get_all_extractors();
    let url_lower = url.to_lowercase();

    for extractor in extractors {
        for extension in extractor.get_extensions() {
            if url_lower.ends_with(extension) {
                return extractor;
            }
        }
    }

    Box::new(RawFileExtractor)
}
pub fn download_data(
    url: &str,
) -> Result<(ProgressBar, Box<dyn Read>), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
        )?
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("#>-"),
    );

    let reader = pb.wrap_read(response);

    Ok((pb, Box::new(reader)))
}
fn get_output_name(url: String, extension: &str) -> String {
    Path::new(&url)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("decompressed_file")
        .trim_end_matches(extension)
        .to_string()
}

pub fn get_reader(
    src: &str,
) -> Result<(Option<ProgressBar>, Box<dyn Read>), Box<dyn std::error::Error>> {
    if Path::new(src).exists() {
        log::info!("processing local file: {}", src);
        let file = File::open(src)?;
        let reader = BufReader::new(file);
        Ok((None, Box::new(reader)))
    } else {
        log::info!("downloading from: {}", src);
        let (pb, reader) = download_data(src)?;
        Ok((Some(pb), reader))
    }
}

pub fn get_all_extractors() -> Vec<Box<dyn Extractor>> {
    vec![
        Box::new(TarGzExtractor),
        Box::new(TarXzExtractor),
        Box::new(TarBz2Extractor),
        Box::new(TarExtractor),
        Box::new(ZipExtractor),
        Box::new(SevenZExtractor),
        Box::new(RarExtractor),
        Box::new(GzExtractor),
        Box::new(Bz2Extractor),
        Box::new(XzExtractor),
    ]
}
