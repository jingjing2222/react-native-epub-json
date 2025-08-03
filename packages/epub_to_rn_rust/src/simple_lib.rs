use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleEpubInfo {
    pub title: Option<String>,
    pub author: Option<String>,
    pub content: String,
}

/// 간단한 테스트용 함수
#[wasm_bindgen(js_name = testEpubConverter)]
pub fn test_epub_converter() -> Result<JsValue, JsValue> {
    let test_data = SimpleEpubInfo {
        title: Some("Test Book".to_string()),
        author: Some("Test Author".to_string()),
        content: "Test content here".to_string(),
    };
    
    serde_wasm_bindgen::to_value(&test_data)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// 문자열을 받아서 JSON 형태로 반환하는 간단한 함수
#[wasm_bindgen(js_name = processText)]
pub fn process_text(input: &str) -> Result<String, JsValue> {
    let result = SimpleEpubInfo {
        title: Some("Processed".to_string()),
        author: Some("System".to_string()),
        content: format!("Processed: {}", input),
    };
    
    serde_json::to_string_pretty(&result)
        .map_err(|e| JsValue::from_str(&format!("JSON error: {}", e)))
}