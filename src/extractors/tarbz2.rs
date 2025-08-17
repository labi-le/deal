use super::Extractor;
use std::io::Read;

pub struct TarBz2Extractor;

impl Extractor for TarBz2Extractor {
    fn extract(
        &self,
        path: String,
        reader: &mut dyn Read,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("tar.bz2 archive detected");
        let archive = bzip2::read::BzDecoder::new(reader);
        let mut tar_archive = tar::Archive::new(archive);
        tar_archive.unpack(path)?;
        Ok(())
    }

    fn get_extensions(&self) -> Vec<&'static str> {
        vec![".tar.bz2", ".tbz2", ".tbz"]
    }
}
