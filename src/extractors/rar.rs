use super::Extractor;
use std::io::{Cursor, Read};

pub struct RarExtractor;

impl Extractor for RarExtractor {
    fn extract(
        &self,
        path: String,
        reader: &mut dyn Read,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("rar archive detected");
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        sevenz_rust::decompress(Cursor::new(buffer), path)
            .map_err(|e| format!("Archive extraction failed: {}", e))?;
        Ok(())
    }

    fn get_extensions(&self) -> Vec<&'static str> {
        vec![".rar"]
    }
}
