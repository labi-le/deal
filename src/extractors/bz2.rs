use super::{Extractor, get_output_name};
use std::fs::File;
use std::io::{Read, copy};

pub struct Bz2Extractor;

impl Extractor for Bz2Extractor {
    fn extract(
        &self,
        path: String,
        reader: &mut dyn Read,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::debug!("bz2 file detected");
        let mut decoder = bzip2::read::BzDecoder::new(reader);

        let output_name = get_output_name(path, ".bz2");
        let mut output_file = File::create(&output_name)?;

        copy(&mut decoder, &mut output_file)?;
        Ok(())
    }

    fn get_extensions(&self) -> Vec<&'static str> {
        vec![".bz2"]
    }
}
