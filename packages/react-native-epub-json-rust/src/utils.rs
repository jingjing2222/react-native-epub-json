use std::fs::File;
use std::io::Write;

/// JSON 문자열을 파일로 저장
pub fn save_json_to_file(json_content: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(json_content.as_bytes())?;
    file.flush()?;
    Ok(())
}

/// 파일 크기를 읽기 쉬운 형태로 포맷
pub fn format_file_size(file_path: &str) -> String {
    if let Ok(metadata) = std::fs::metadata(file_path) {
        let size_kb = metadata.len() / 1024;
        let size_mb = size_kb as f64 / 1024.0;
        if size_mb > 1.0 {
            format!("{:.1} MB", size_mb)
        } else {
            format!("{} KB", size_kb)
        }
    } else {
        "Unknown".to_string()
    }
} 