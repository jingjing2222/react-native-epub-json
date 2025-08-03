use epub::doc::EpubDoc;
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};
use std::io::{Read, Seek, Cursor};

// WASM 환경에서 console.log 사용을 위한 매크로
#[cfg(target_arch = "wasm32")]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! log {
    ($($t:tt)*) => {
        println!($($t)*);
    }
}

use crate::types::*;
use crate::css_parser::parse_css_to_rn_styles;
use crate::html_parser::{parse_html_to_rn_nodes, extract_title_from_html};

/// EPUB 바이트에서 완전한 정보를 추출하여 React Native 구조로 변환
pub fn extract_complete_epub_info_from_bytes(epub_bytes: &[u8]) -> Result<CompleteEpubInfo, Box<dyn std::error::Error>> {
    let cursor = Cursor::new(epub_bytes);
    let mut doc = EpubDoc::from_reader(cursor)?;
    extract_epub_data(&mut doc)
}

/// EPUB 파일에서 완전한 정보를 추출하여 React Native 구조로 변환
pub fn extract_complete_epub_info(epub_path: &str) -> Result<CompleteEpubInfo, Box<dyn std::error::Error>> {
    let mut doc = EpubDoc::new(epub_path)?;
    extract_epub_data(&mut doc)
}

/// 공통 EPUB 데이터 추출 로직
fn extract_epub_data<R: Read + Seek>(doc: &mut EpubDoc<R>) -> Result<CompleteEpubInfo, Box<dyn std::error::Error>> {
    
    // 메타데이터 추출
    let metadata = extract_metadata(doc);
    
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
    
    // CSS 스타일 및 이미지 추출
    let resources_map = doc.resources.clone();
    let (styles, images) = extract_styles_and_images(doc, &resources_map);
    
    // 챕터 내용을 RN 노드 구조로 변환
    let chapters = extract_chapters(doc, &spine_items, &resources_map, &styles, &images);
    
    Ok(CompleteEpubInfo {
        metadata,
        structure,
        toc,
        spine,
        styles,
        images,
        chapters,
    })
}

/// 메타데이터 추출
fn extract_metadata<R: Read + Seek>(doc: &mut EpubDoc<R>) -> EpubMetadata {
    EpubMetadata {
        title: doc.mdata("title"),
        author: doc.mdata("creator"),
        language: doc.mdata("language"),
        publisher: doc.mdata("publisher"),
        description: doc.mdata("description"),
        date: doc.mdata("date"),
        identifier: doc.mdata("identifier"),
        rights: doc.mdata("rights"),
        subject: doc.mdata("subject"),
    }
}

/// CSS 스타일과 이미지 추출
fn extract_styles_and_images<R: Read + Seek>(
    doc: &mut EpubDoc<R>, 
    resources_map: &HashMap<String, (std::path::PathBuf, String)>
) -> (HashMap<String, RnStyles>, HashMap<String, String>) {
    let mut styles = HashMap::new();
    let mut images = HashMap::new();
    
    let mut css_files = Vec::new();
    let mut image_files = Vec::new();
    let mut other_files = Vec::new();
    
    // 리소스 타입별 분류
    for (id, (path, mime_type)) in resources_map.iter() {
        if mime_type == "text/css" {
            css_files.push((id, path));
        } else if mime_type.starts_with("image/") {
            image_files.push((id, path, mime_type));
        } else {
            other_files.push((id, mime_type));
        }
    }
    
    // CSS 파일들을 RN 스타일로 변환
    for (index, (id, path)) in css_files.iter().enumerate() {
        if let Some((css_content, _)) = doc.get_resource_str(id) {
            if css_content.len() > 0 {
                let parsed_styles = parse_css_to_rn_styles(&css_content);
                let styles_count = parsed_styles.len();
                styles.extend(parsed_styles);
                
            } else {
                log!("   ⚠️  CSS file is empty");
            }
        } else {
            log!("   ❌ Failed to read CSS file: {}", id);
        }
    }
    
    
    for (index, (id, path, mime_type)) in image_files.iter().enumerate() {
        if let Some((data, _)) = doc.get_resource(id) {
            let base64_data = general_purpose::STANDARD.encode(&data);
            let data_uri = format!("data:{};base64,{}", mime_type, base64_data);
            let size_kb = data.len() / 1024;
            
            images.insert(id.to_string(), data_uri);
        } else {
            log!("   ❌ Failed to read image: {}", id);
        }
    }
    
    (styles, images)
}

/// 챕터들을 RN 노드 구조로 변환
fn extract_chapters<R: Read + Seek>(
    doc: &mut EpubDoc<R>,
    spine_items: &[epub::doc::SpineItem],
    resources_map: &HashMap<String, (std::path::PathBuf, String)>,
    styles: &HashMap<String, RnStyles>,
    images: &HashMap<String, String>
) -> Vec<ChapterStructure> {
    // 챕터 정보 먼저 수집
    let mut chapter_info: Vec<(usize, String)> = Vec::new();
    for (index, spine_item) in spine_items.iter().enumerate() {
        if let Some(resource) = resources_map.get(&spine_item.idref) {
            if resource.1 == "application/xhtml+xml" {
                chapter_info.push((index, spine_item.idref.clone()));
            }
        }
    }
    
    // 챕터 내용을 RN 노드 구조로 변환
    let mut chapters = Vec::new();
    for (spine_index, idref) in chapter_info {
        if let Some((html_content, _)) = doc.get_resource_str(&idref) {
            let rn_node = parse_html_to_rn_nodes(&html_content, styles, images);
            let title = extract_title_from_html(&html_content);
            
            chapters.push(ChapterStructure {
                spine_index,
                idref,
                title,
                content: rn_node,
            });
        }
    }
    
    chapters
} 