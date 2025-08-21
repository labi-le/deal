
#[cfg(test)]
mod tests {
    use super::super::TarBz2Extractor;
    use crate::extractors::Extractor;
    use bzip2::write::BzEncoder;
    use bzip2::Compression;
    use std::io::{Cursor, Write};
    use tar::Header;
    use tempfile::tempdir;

    fn create_in_memory_tarbz2(file_name: &str, content: &[u8]) -> Vec<u8> {
        let mut tar_builder = tar::Builder::new(Vec::new());
        let mut header = Header::new_gnu();
        header.set_size(content.len() as u64);
        header.set_cksum();
        tar_builder
            .append_data(&mut header, file_name, content)
            .unwrap();
        let tar_data = tar_builder.into_inner().unwrap();

        let mut encoder = BzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&tar_data).unwrap();
        encoder.finish().unwrap()
    }

    #[test]
    fn test_tarbz2_extraction() {
        let dir = tempdir().unwrap();
        let extract_path = dir.path().to_str().unwrap().to_string();
        let file_content = b"hello tar.bz2";
        let file_name = "test.txt";

        let buffer = create_in_memory_tarbz2(file_name, file_content);

        let extractor = TarBz2Extractor;
        let mut reader = Cursor::new(buffer);
        extractor
            .extract(extract_path.clone(), &mut reader)
            .unwrap();

        let extracted_file_path = dir.path().join(file_name);
        assert!(extracted_file_path.exists());
        let content = std::fs::read_to_string(extracted_file_path).unwrap();
        assert_eq!(content, String::from_utf8_lossy(file_content));
    }
}
