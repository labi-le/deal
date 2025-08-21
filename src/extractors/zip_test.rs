
#[cfg(test)]
mod tests {
    use super::super::ZipExtractor;
    use crate::extractors::Extractor;
    use std::io::{Cursor, Write};
    use tempfile::tempdir;
    use zip::write::{FileOptions, ZipWriter};

    fn create_in_memory_zip(file_name: &str, content: &[u8]) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut zip = ZipWriter::new(Cursor::new(&mut buffer));
        let options: FileOptions<'_, ()> = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        zip.start_file(file_name, options).unwrap();
        zip.write_all(content).unwrap();
        zip.finish().unwrap();
        buffer
    }

    #[test]
    fn test_zip_extraction() {
        let dir = tempdir().unwrap();
        let extract_path = dir.path().to_str().unwrap().to_string();
        let file_content = b"hello world";
        let file_name = "test.txt";

        let buffer = create_in_memory_zip(file_name, file_content);

        let extractor = ZipExtractor;
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
