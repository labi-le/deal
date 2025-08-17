use super::Extractor;
use std::io::{Cursor, Read};

pub struct ZipExtractor;

impl Extractor for ZipExtractor {
    fn extract(
        &self,
        path: String,
        reader: &mut dyn Read,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("zip archive detected");
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let mut archive = zip::ZipArchive::new(Cursor::new(buffer))?;
        archive.extract(path)?;
        Ok(())
    }

    fn get_extensions(&self) -> Vec<&'static str> {
        vec![".zip"]
    }
}
