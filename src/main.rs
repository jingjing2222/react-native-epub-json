use epub_to_rn::{convert_epub_to_rn_json, CompleteEpubInfo};
use epub_to_rn::utils::format_file_size;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // 사용법 체크
    if args.len() < 3 {
        print_usage(&args[0]);
        std::process::exit(1);
    }
    
    let epub_path = &args[1];
    let output_path = &args[2];
    
    println!("🚀 EPUB → React Native 변환 시작...");
    println!("   📖 입력: {}", epub_path);
    println!("   💾 출력: {}", output_path);
    
    match convert_epub_to_rn_json(epub_path, output_path) {
        Ok(epub_info) => {
            println!("✅ React Native 구조로 성공적으로 변환되어 '{}' 파일로 저장되었습니다!", output_path);
            
            // 변환 결과 요약
            print_conversion_summary(&epub_info, output_path);
            
            println!("\n🎉 React Native에서 바로 렌더링 가능한 구조화된 데이터가 준비되었습니다!");
        }
        Err(e) => {
            eprintln!("❌ 변환 실패: {}", e);
            std::process::exit(1);
        }
    }
}

/// 사용법 출력
fn print_usage(program_name: &str) {
    println!("📚 EPUB to React Native JSON Converter");
    println!();
    println!("사용법:");
    println!("  {} <input.epub> <output.json>", program_name);
    println!();
    println!("예시:");
    println!("  {} /path/to/book.epub /path/to/output.json", program_name);
    println!("  {} ./book.epub ./book.json", program_name);
    println!();
    println!("기능:");
    println!("  🚀 HTML/CSS를 React Native 컴포넌트 구조로 변환");
    println!("  🎨 CSS 스타일을 RN StyleSheet로 자동 변환");
    println!("  🖼️  이미지를 base64로 인코딩하여 self-contained JSON 생성");
    println!("  📱 웹뷰 없이 완전한 네이티브 렌더링 지원");
}

/// 변환 결과 요약 출력
fn print_conversion_summary(epub_info: &CompleteEpubInfo, output_path: &str) {
    println!("\n📊 변환된 정보:");
    println!("   📚 제목: {:?}", epub_info.metadata.title.as_deref().unwrap_or("N/A"));
    println!("   ✍️  작가: {:?}", epub_info.metadata.author.as_deref().unwrap_or("N/A"));
    println!("   🌐 언어: {:?}", epub_info.metadata.language.as_deref().unwrap_or("N/A"));
    println!("   📅 출간일: {:?}", epub_info.metadata.date.as_deref().unwrap_or("N/A"));
    println!("   📄 챕터 수: {}", epub_info.chapters.len());
    println!("   🎨 스타일 규칙: {} 개", epub_info.styles.len());
    println!("   🖼️  이미지: {} 개", epub_info.images.len());
    println!("   📦 파일 크기: {}", format_file_size(output_path));
    
    // 챕터별 정보
    if !epub_info.chapters.is_empty() {
        println!("\n📖 챕터 목록:");
        for (i, chapter) in epub_info.chapters.iter().enumerate().take(5) {
            let title = chapter.title.as_deref().unwrap_or("제목 없음");
            println!("   {}. {}", i + 1, title);
        }
        if epub_info.chapters.len() > 5 {
            println!("   ... 그리고 {} 개 더", epub_info.chapters.len() - 5);
        }
    }
    
    // 스타일 샘플
    if !epub_info.styles.is_empty() {
        println!("\n🎨 주요 스타일:");
        for (name, _) in epub_info.styles.iter().take(3) {
            println!("   • {}", name);
        }
        if epub_info.styles.len() > 3 {
            println!("   ... 그리고 {} 개 더", epub_info.styles.len() - 3);
        }
    }
}