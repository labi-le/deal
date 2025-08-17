use super::Extractor;
use std::io::Read;

pub struct TarXzExtractor;

impl Extractor for TarXzExtractor {
    fn extract(
        &self,
        path: String,
        reader: &mut dyn Read,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("tar.xz archive detected");
        let archive = xz2::read::XzDecoder::new(reader);
        let mut tar_archive = tar::Archive::new(archive);
        tar_archive.unpack(path)?;
        Ok(())
    }

    fn get_extensions(&self) -> Vec<&'static str> {
        vec![".tar.xz", ".txz"]
    }
}
