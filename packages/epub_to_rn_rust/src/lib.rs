//! # EPUB to React Native Converter
//! 
//! 이 라이브러리는 EPUB 파일을 React Native에서 바로 렌더링할 수 있는 
//! 구조화된 JSON으로 변환합니다.
//! 
//! ## 특징
//! 
//! - 🚀 **완전한 파싱**: HTML, CSS, 이미지를 모두 Rust에서 처리
//! - 📱 **React Native 최적화**: 웹뷰 없이 네이티브 컴포넌트로 렌더링
//! - 💾 **Self-contained**: 모든 리소스가 JSON에 임베드됨
//! - 🎨 **스타일 변환**: CSS → React Native StyleSheet 자동 변환
//! 
//! ## 사용법
//! 
//! ```rust
//! use epub_test::epub_extractor::extract_complete_epub_info;
//! 
//! let epub_info = extract_complete_epub_info("book.epub")?;
//! let json = serde_json::to_string_pretty(&epub_info)?;
//! ```

pub mod types;
pub mod css_parser;
pub mod html_parser;
pub mod utils;
pub mod epub_extractor;

// 주요 타입들 재밷출
pub use types::*;
pub use epub_extractor::{extract_complete_epub_info, extract_complete_epub_info_from_bytes};

/// CLI와 WASM에서 공통으로 사용할 변환 함수
pub fn convert_epub_to_rn_json(epub_path: &str, output_path: &str) -> Result<CompleteEpubInfo, Box<dyn std::error::Error>> {
    let epub_info = extract_complete_epub_info(epub_path)?;
    let json = serde_json::to_string_pretty(&epub_info)?;
    std::fs::write(output_path, json)?;
    Ok(epub_info)
}

// WASM 바인딩을 위한 imports
use wasm_bindgen::prelude::*;

// wee_alloc을 글로벌 할당자로 설정 (크기 최적화)
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 패닉 시 콘솔에 더 나은 에러 메시지 표시
#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// Node.js에서 사용할 EPUB → JSON 변환 함수
/// 
/// # JavaScript 사용법
/// ```javascript
/// import { epubToJson } from 'epub-to-rn';
/// 
/// const result = epubToJson('path/to/book.epub', './output');
/// console.log(result.metadata.title);
/// ```
#[wasm_bindgen(js_name = epubToJson)]
pub fn epub_to_json(epub_path: &str, output_dir: &str) -> Result<JsValue, JsValue> {
    // WASM에서는 파일 시스템 접근을 시도하되, 실패하면 그냥 진행
    // Node.js 환경에서는 정상 작동할 수 있음
    
    // 3. EPUB 추출 및 변환
    let epub_info = extract_complete_epub_info(epub_path)
        .map_err(|e| JsValue::from_str(&format!("EPUB 파일 처리 실패: {}", e)))?;
    
    // 4. JSON 파일 저장 시도
    let output_path = format!("{}/book.json", output_dir.trim_end_matches('/'));
    let json = serde_json::to_string_pretty(&epub_info)
        .map_err(|e| JsValue::from_str(&format!("JSON 직렬화 실패: {}", e)))?;
    
    // 출력 디렉토리 생성 시도 (실패해도 계속 진행)
    let _ = std::fs::create_dir_all(output_dir);
    
    // 파일 저장 시도 (실패해도 계속 진행)
    let _ = std::fs::write(&output_path, &json);
    
    // 5. JavaScript 객체로 반환
    serde_wasm_bindgen::to_value(&epub_info)
        .map_err(|e| JsValue::from_str(&format!("WASM 직렬화 실패: {}", e)))
}

/// 메모리에서 EPUB 바이트를 JSON 문자열로 변환 (파일 저장 없이)
/// 
/// # JavaScript 사용법
/// ```javascript
/// import { epubBytesToJson } from 'epub-to-rn';
/// 
/// const fileBuffer = fs.readFileSync('book.epub');
/// const result = epubBytesToJson(new Uint8Array(fileBuffer));
/// ```
#[wasm_bindgen(js_name = epubBytesToJson)]
pub fn epub_bytes_to_json(epub_bytes: &[u8]) -> Result<JsValue, JsValue> {
    // 메모리에서 직접 처리 (임시 파일 없음)
    let epub_info = extract_complete_epub_info_from_bytes(epub_bytes)
        .map_err(|e| JsValue::from_str(&format!("EPUB 파일 처리 실패: {}", e)))?;
    
    // JavaScript 객체로 반환
    serde_wasm_bindgen::to_value(&epub_info)
        .map_err(|e| JsValue::from_str(&format!("WASM 직렬화 실패: {}", e)))
}

/// 간단한 버전: EPUB 경로를 받아서 JSON 문자열만 반환
/// 
/// # JavaScript 사용법
/// ```javascript
/// import { epubToJsonString } from 'epub-to-rn';
/// 
/// const jsonString = epubToJsonString('book.epub');
/// const data = JSON.parse(jsonString);
/// ```
#[wasm_bindgen(js_name = epubToJsonString)]
pub fn epub_to_json_string(epub_path: &str) -> Result<String, JsValue> {
    // WASM에서는 파일 존재 확인을 생략하고 바로 처리 시도
    let epub_info = extract_complete_epub_info(epub_path)
        .map_err(|e| JsValue::from_str(&format!("EPUB 파일 처리 실패: {}", e)))?;
    
    serde_json::to_string_pretty(&epub_info)
        .map_err(|e| JsValue::from_str(&format!("JSON 직렬화 실패: {}", e)))
} 