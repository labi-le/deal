
#[cfg(test)]
mod tests {
    use super::super::GzExtractor;
    use crate::extractors::Extractor;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::{Cursor, Write};
    use tempfile::tempdir;

    fn create_in_memory_gz(content: &[u8]) -> Vec<u8> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(content).unwrap();
        encoder.finish().unwrap()
    }

    #[test]
    fn test_gz_extraction() {
        let dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        let file_content = b"hello gz";
        let file_name = "test.txt";
        let archive_name = format!("{}.gz", file_name);

        let buffer = create_in_memory_gz(file_content);

        let extractor = GzExtractor;
        let mut reader = Cursor::new(buffer);
        extractor.extract(archive_name, &mut reader).unwrap();

        let extracted_file_path = std::path::Path::new(file_name);
        assert!(extracted_file_path.exists());
        let content = std::fs::read_to_string(extracted_file_path).unwrap();
        assert_eq!(content, String::from_utf8_lossy(file_content));

        std::env::set_current_dir(original_dir).unwrap();
    }
}
