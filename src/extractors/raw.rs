use super::Extractor;
use std::fs::File;
use std::io::{Read, copy};
use std::path::Path;

pub struct RawFileExtractor;

impl Extractor for RawFileExtractor {
    fn extract(
        &self,
        path: String,
        reader: &mut dyn Read,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_name = Path::new(&path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("downloaded_file");

        log::debug!("unknown archive type. saving as a raw file: {}", file_name);

        let mut dest_file = File::create(file_name)?;
        copy(reader, &mut dest_file)?;

        Ok(())
    }

    fn get_extensions(&self) -> Vec<&'static str> {
        vec![]
    }
}
