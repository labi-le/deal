use super::{Extractor, get_output_name};
use std::fs::File;
use std::io::{Read, copy};

pub struct GzExtractor;

impl Extractor for GzExtractor {
    fn extract(
        &self,
        path: String,
        reader: &mut dyn Read,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("gz archive detected");
        let mut decoder = flate2::read::GzDecoder::new(reader);

        let output_name = get_output_name(path, ".gz");
        let mut output_file = File::create(&output_name)?;
        copy(&mut decoder, &mut output_file)?;
        Ok(())
    }

    fn get_extensions(&self) -> Vec<&'static str> {
        vec![".gz"]
    }
}
