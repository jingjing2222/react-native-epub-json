use epub_test::{extract_complete_epub_info, CompleteEpubInfo};
use epub_test::utils::{save_json_to_file, format_file_size};

fn main() {
    let epub_path = "/Users/kimhyeongjeong/Desktop/code/ebook/hemingway-old-man-and-the-sea.epub";
    let output_path = "epub_complete.json";
    
    println!("🚀 EPUB → React Native 변환 시작...");
    
    match extract_complete_epub_info(epub_path) {
        Ok(epub_info) => {
            println!("📖 EPUB 파일 분석 완료!");
            
            match serde_json::to_string_pretty(&epub_info) {
                Ok(json) => {
                    match save_json_to_file(&json, output_path) {
                        Ok(_) => {
                            println!("✅ React Native 구조로 성공적으로 변환되어 '{}' 파일로 저장되었습니다!", output_path);
                            
                            // 변환 결과 요약
                            print_conversion_summary(&epub_info, output_path);
                            
                            println!("\n🎉 React Native에서 바로 렌더링 가능한 구조화된 데이터가 준비되었습니다!");
                        }
                        Err(e) => println!("❌ 파일 저장 실패: {}", e),
                    }
                }
                Err(e) => println!("❌ JSON 직렬화 에러: {}", e),
            }
        }
        Err(e) => println!("❌ EPUB 파일 처리 에러: {}", e),
    }
}

/// 변환 결과 요약 출력
fn print_conversion_summary(epub_info: &CompleteEpubInfo, output_path: &str) {
    println!("\n📊 변환된 정보:");
    println!("   📚 제목: {:?}", epub_info.metadata.title.as_deref().unwrap_or("N/A"));
    println!("   ✍️  작가: {:?}", epub_info.metadata.author.as_deref().unwrap_or("N/A"));
    println!("   📄 챕터 수: {}", epub_info.chapters.len());
    println!("   🎨 스타일 규칙: {} 개", epub_info.styles.len());
    println!("   🖼️  이미지: {} 개", epub_info.images.len());
    println!("   📦 파일 크기: {}", format_file_size(output_path));
}