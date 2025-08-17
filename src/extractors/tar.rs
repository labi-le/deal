use super::Extractor;
use std::io::Read;

pub struct TarExtractor;

impl Extractor for TarExtractor {
    fn extract(
        &self,
        path: String,
        reader: &mut dyn Read,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("tar archive detected");
        let mut archive = tar::Archive::new(reader);
        archive.unpack(path)?;
        Ok(())
    }

    fn get_extensions(&self) -> Vec<&'static str> {
        vec![".tar"]
    }
}
