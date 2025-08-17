use super::{Extractor, get_output_name};
use std::fs::File;
use std::io::{Read, copy};

pub struct XzExtractor;

impl Extractor for XzExtractor {
    fn extract(
        &self,
        path: String,
        reader: &mut dyn Read,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("xz file detected");
        let mut decoder = xz2::read::XzDecoder::new(reader);

        let output_name = get_output_name(path, ".xz");
        let mut output_file = File::create(&output_name)?;

        copy(&mut decoder, &mut output_file)?;

        Ok(())
    }

    fn get_extensions(&self) -> Vec<&'static str> {
        vec![".xz"]
    }
}
