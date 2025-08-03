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
pub use epub_extractor::extract_complete_epub_info; 