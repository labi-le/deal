
#[cfg(test)]
mod tests {
    use super::super::TarXzExtractor;
    use crate::extractors::Extractor;
    use std::io::{Cursor, Write};
    use tar::Header;
    use tempfile::tempdir;
    use xz2::write::XzEncoder;

    fn create_in_memory_tarxz(file_name: &str, content: &[u8]) -> Vec<u8> {
        let mut tar_builder = tar::Builder::new(Vec::new());
        let mut header = Header::new_gnu();
        header.set_size(content.len() as u64);
        header.set_cksum();
        tar_builder
            .append_data(&mut header, file_name, content)
            .unwrap();
        let tar_data = tar_builder.into_inner().unwrap();

        let mut encoder = XzEncoder::new(Vec::new(), 6);
        encoder.write_all(&tar_data).unwrap();
        encoder.finish().unwrap()
    }

    #[test]
    fn test_tarxz_extraction() {
        let dir = tempdir().unwrap();
        let extract_path = dir.path().to_str().unwrap().to_string();
        let file_content = b"hello tar.xz";
        let file_name = "test.txt";

        let buffer = create_in_memory_tarxz(file_name, file_content);

        let extractor = TarXzExtractor;
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
