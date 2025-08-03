use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use base64::{Engine as _, engine::general_purpose};

#[derive(Serialize, Deserialize, Debug)]
struct CompleteEpubInfo {
    metadata: EpubMetadata,
    structure: EpubStructure,
    toc: Vec<TocItem>,
    spine: Vec<SpineItemInfo>,
    resources: HashMap<String, ResourceContent>, // 실제 내용 포함
    chapters: Vec<ChapterContent>,
}

#[derive(Serialize, Deserialize, Debug)]
struct EpubMetadata {
    title: Option<String>,
    author: Option<String>,
    language: Option<String>,
    publisher: Option<String>,
    description: Option<String>,
    date: Option<String>,
    identifier: Option<String>,
    rights: Option<String>,
    subject: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct EpubStructure {
    spine_count: usize,
    resource_count: usize,
    toc_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct TocItem {
    label: String,
    content_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpineItemInfo {
    idref: String,
    id: Option<String>,
    properties: Option<String>,
    linear: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResourceContent {
    path: String,
    mime_type: String,
    content: ResourceData, // 실제 내용
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ResourceData {
    Text { data: String },           // CSS, XHTML 등
    Binary { data: String },        // 이미지 등 (base64)
    Error { message: String },      // 읽기 실패한 경우
}

#[derive(Serialize, Deserialize, Debug)]
struct ChapterContent {
    spine_index: usize,
    idref: String,
    path: String,
    mime_type: String,
    content: Option<String>,
    error: Option<String>,
}

fn main() {
    let epub_path = "/Users/kimhyeongjeong/Desktop/code/ebook/hemingway-old-man-and-the-sea.epub";
    let output_path = "epub_complete.json";
    
    match extract_complete_epub_info(epub_path) {
        Ok(epub_info) => {
            match serde_json::to_string_pretty(&epub_info) {
                Ok(json) => {
                    match save_json_to_file(&json, output_path) {
                        Ok(_) => {
                            println!("✅ EPUB 완전한 내용이 성공적으로 '{}' 파일로 저장되었습니다!", output_path);
                            println!("📊 저장된 정보:");
                            println!("   - 메타데이터: 제목, 저자, 언어 등");
                            println!("   - 구조 정보: {} 개 챕터, {} 개 리소스", 
                                epub_info.structure.spine_count, 
                                epub_info.structure.resource_count);
                            println!("   - 목차: {} 개 항목", epub_info.structure.toc_count);
                            println!("   - 실제 콘텐츠: {} 개 챕터의 전체 내용", epub_info.chapters.len());
                            println!("   - 모든 리소스의 실제 내용 (이미지는 base64로 인코딩)");
                            
                            // 리소스 타입별 개수 출력
                            let mut text_count = 0;
                            let mut binary_count = 0;
                            let mut error_count = 0;
                            
                            for resource in epub_info.resources.values() {
                                match &resource.content {
                                    ResourceData::Text { .. } => text_count += 1,
                                    ResourceData::Binary { .. } => binary_count += 1,
                                    ResourceData::Error { .. } => error_count += 1,
                                }
                            }
                            
                            println!("   - 텍스트 리소스: {} 개 (CSS, XHTML 등)", text_count);
                            println!("   - 바이너리 리소스: {} 개 (이미지 등, base64 인코딩)", binary_count);
                            if error_count > 0 {
                                println!("   - 읽기 실패: {} 개", error_count);
                            }
                            
                            // 파일 크기 확인
                            if let Ok(metadata) = std::fs::metadata(output_path) {
                                let size_kb = metadata.len() / 1024;
                                let size_mb = size_kb as f64 / 1024.0;
                                if size_mb > 1.0 {
                                    println!("   - 파일 크기: {:.1} MB", size_mb);
                                } else {
                                    println!("   - 파일 크기: {} KB", size_kb);
                                }
                            }
                            
                            println!("\n🎉 이제 이 JSON 파일만으로 React Native에서 완전한 EPUB 렌더링이 가능합니다!");
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

fn save_json_to_file(json_content: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(json_content.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn extract_complete_epub_info(epub_path: &str) -> Result<CompleteEpubInfo, Box<dyn std::error::Error>> {
    let mut doc = EpubDoc::new(epub_path)?;
    
    // 메타데이터 추출
    let metadata = EpubMetadata {
        title: doc.mdata("title"),
        author: doc.mdata("creator"),
        language: doc.mdata("language"),
        publisher: doc.mdata("publisher"),
        description: doc.mdata("description"),
        date: doc.mdata("date"),
        identifier: doc.mdata("identifier"),
        rights: doc.mdata("rights"),
        subject: doc.mdata("subject"),
    };
    
    // 구조 정보
    let structure = EpubStructure {
        spine_count: doc.spine.len(),
        resource_count: doc.resources.len(),
        toc_count: doc.toc.len(),
    };
    
    // TOC 정보
    let toc: Vec<TocItem> = doc.toc.iter().map(|item| TocItem {
        label: item.label.clone(),
        content_path: item.content.display().to_string(),
    }).collect();
    
    // Spine 정보
    let spine_items = doc.spine.clone();
    let spine: Vec<SpineItemInfo> = spine_items.iter().map(|item| SpineItemInfo {
        idref: item.idref.clone(),
        id: item.id.clone(),
        properties: item.properties.clone(),
        linear: item.linear,
    }).collect();
    
    // 모든 리소스의 실제 내용 추출
    let resources_map = doc.resources.clone();
    let mut resources: HashMap<String, ResourceContent> = HashMap::new();
    
    for (id, (path, mime_type)) in resources_map.iter() {
        let content = extract_resource_content(&mut doc, id, mime_type);
        
        resources.insert(id.clone(), ResourceContent {
            path: path.display().to_string(),
            mime_type: mime_type.clone(),
            content,
        });
    }
    
    // 챕터 정보 먼저 수집 (content 제외)
    let mut chapter_info: Vec<(usize, String, String, String)> = Vec::new();
    for (index, spine_item) in spine_items.iter().enumerate() {
        if let Some(resource) = resources_map.get(&spine_item.idref) {
            let mime_type = resource.1.clone();
            let path = resource.0.display().to_string();
            chapter_info.push((index, spine_item.idref.clone(), path, mime_type));
        }
    }
    
    // 챕터 내용 추출
    let mut chapters = Vec::new();
    for (spine_index, idref, path, mime_type) in chapter_info {
        let (content, error) = if mime_type == "application/xhtml+xml" {
            match doc.get_resource_str(&idref) {
                Some((content, _)) => (Some(content), None),
                None => (None, Some("리소스를 찾을 수 없음".to_string())),
            }
        } else {
            (None, Some(format!("XHTML이 아닌 파일: {}", mime_type)))
        };
        
        chapters.push(ChapterContent {
            spine_index,
            idref,
            path,
            mime_type,
            content,
            error,
        });
    }
    
    Ok(CompleteEpubInfo {
        metadata,
        structure,
        toc,
        spine,
        resources,
        chapters,
    })
}

fn extract_resource_content<R: std::io::Read + std::io::Seek>(doc: &mut EpubDoc<R>, resource_id: &str, mime_type: &str) -> ResourceData {
    match doc.get_resource(resource_id) {
        Some((data, _)) => {  // 튜플에서 데이터 부분만 추출
            if mime_type.starts_with("text/") || 
               mime_type == "application/xhtml+xml" || 
               mime_type == "application/x-dtbncx+xml" {
                // 텍스트 기반 리소스
                match String::from_utf8(data) {
                    Ok(text) => ResourceData::Text { data: text },
                    Err(_) => ResourceData::Error { 
                        message: "텍스트 디코딩 실패".to_string() 
                    },
                }
            } else {
                // 바이너리 리소스 (이미지 등) - base64로 인코딩
                let encoded = general_purpose::STANDARD.encode(&data);
                ResourceData::Binary { data: encoded }
            }
        }
        None => ResourceData::Error { 
            message: "리소스를 읽을 수 없음".to_string() 
        },
    }
}