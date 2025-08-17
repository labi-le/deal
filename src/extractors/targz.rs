use super::Extractor;
use std::io::Read;

pub struct TarGzExtractor;

impl Extractor for TarGzExtractor {
    fn extract(
        &self,
        path: String,
        reader: &mut dyn Read,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("tar.gz archive detected");
        let archive = flate2::read::GzDecoder::new(reader);
        let mut tar_archive = tar::Archive::new(archive);
        tar_archive.unpack(path)?;
        Ok(())
    }

    fn get_extensions(&self) -> Vec<&'static str> {
        vec![".tar.gz", ".tgz"]
    }
}
