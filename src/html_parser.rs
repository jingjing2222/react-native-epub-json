use std::collections::HashMap;
use scraper::{Html, Selector};
use crate::types::{RnNode, RnStyles};
use crate::css_parser::{parse_css_declarations, merge_styles};

/// HTML을 React Native 노드 구조로 변환
pub fn parse_html_to_rn_nodes(html: &str, styles: &HashMap<String, RnStyles>, images: &HashMap<String, String>) -> RnNode {
    let document = Html::parse_document(html);
    
    // body 태그 찾기
    let body_selector = Selector::parse("body").unwrap();
    if let Some(body) = document.select(&body_selector).next() {
        convert_element_to_rn_node(body, styles, images)
    } else {
        // body가 없으면 전체 문서를 View로 감싸기
        let mut children = Vec::new();
        let root_selector = Selector::parse("html").unwrap();
        
        for element in document.select(&root_selector) {
            let child_node = convert_element_to_rn_node(element, styles, images);
            children.push(child_node);
        }
        
        if children.is_empty() {
            // 아무것도 없으면 간단한 텍스트 노드
            RnNode::Text { 
                content: "Empty content".to_string(), 
                styles: None 
            }
        } else {
            RnNode::View { children, styles: None }
        }
    }
}

/// HTML 요소를 React Native 노드로 변환
pub fn convert_element_to_rn_node(element: scraper::ElementRef, styles: &HashMap<String, RnStyles>, images: &HashMap<String, String>) -> RnNode {
    let tag_name = element.value().name();
    let mut children = Vec::new();
    
    // 자식 노드들 처리
    for child in element.children() {
        if let Some(text) = child.value().as_text() {
            let content = text.trim();
            if !content.is_empty() {
                children.push(RnNode::Text { 
                    content: content.to_string(), 
                    styles: None 
                });
            }
        } else if child.value().as_element().is_some() {
            if let Some(child_element) = scraper::ElementRef::wrap(child) {
                let child_node = convert_element_to_rn_node(child_element, styles, images);
                children.push(child_node);
            }
        }
    }
    
    // 인라인 스타일 추출
    let inline_style = extract_inline_styles(element);
    
    // CSS 클래스 스타일 추출
    let class_style = extract_class_styles(element, styles);
    
    // 스타일 병합
    let merged_style = merge_styles(class_style, inline_style);
    
    match tag_name {
        "p" | "div" | "section" | "article" | "header" | "footer" => {
            RnNode::View { children, styles: merged_style }
        }
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            let mut heading_style = merged_style.unwrap_or_else(|| create_empty_style());
            
            // 헤딩 기본 스타일
            heading_style.fontWeight = Some("bold".to_string());
            heading_style.fontSize = Some(match tag_name {
                "h1" => 24.0,
                "h2" => 20.0,
                "h3" => 18.0,
                "h4" => 16.0,
                "h5" => 14.0,
                "h6" => 12.0,
                _ => 16.0,
            });
            
            RnNode::View { children, styles: Some(heading_style) }
        }
        "img" => {
            if let Some(src) = element.value().attr("src") {
                let alt = element.value().attr("alt").map(|s| s.to_string());
                
                // 이미지 소스가 리소스 맵에 있는지 확인
                let source = if let Some(image_data) = images.get(src) {
                    image_data.clone()
                } else {
                    // 상대 경로에서 파일명만 추출해서 다시 시도
                    let filename = src.split('/').last().unwrap_or(src);
                    images.get(filename).cloned().unwrap_or_else(|| src.to_string())
                };
                
                RnNode::Image { source, alt, styles: merged_style }
            } else {
                RnNode::View { children, styles: merged_style }
            }
        }
        "strong" | "b" => {
            let mut bold_style = merged_style.unwrap_or_else(|| create_empty_style());
            bold_style.fontWeight = Some("bold".to_string());
            
            if children.len() == 1 {
                if let RnNode::Text { content, .. } = &children[0] {
                    return RnNode::Text { content: content.clone(), styles: Some(bold_style) };
                }
            }
            RnNode::View { children, styles: Some(bold_style) }
        }
        "em" | "i" => {
            let mut italic_style = merged_style.unwrap_or_else(|| create_empty_style());
            italic_style.fontStyle = Some("italic".to_string());
            
            if children.len() == 1 {
                if let RnNode::Text { content, .. } = &children[0] {
                    return RnNode::Text { content: content.clone(), styles: Some(italic_style) };
                }
            }
            RnNode::View { children, styles: Some(italic_style) }
        }
        "u" => {
            let mut underline_style = merged_style.unwrap_or_else(|| create_empty_style());
            underline_style.textDecorationLine = Some("underline".to_string());
            
            if children.len() == 1 {
                if let RnNode::Text { content, .. } = &children[0] {
                    return RnNode::Text { content: content.clone(), styles: Some(underline_style) };
                }
            }
            RnNode::View { children, styles: Some(underline_style) }
        }
        _ => {
            RnNode::View { children, styles: merged_style }
        }
    }
}

/// HTML에서 제목 추출
pub fn extract_title_from_html(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let title_selector = Selector::parse("title").unwrap();
    
    if let Some(title_element) = document.select(&title_selector).next() {
        Some(title_element.text().collect::<Vec<_>>().join(""))
    } else {
        // h1 태그에서 제목 찾기
        let h1_selector = Selector::parse("h1").unwrap();
        if let Some(h1_element) = document.select(&h1_selector).next() {
            Some(h1_element.text().collect::<Vec<_>>().join(""))
        } else {
            None
        }
    }
}

/// 인라인 스타일 추출
fn extract_inline_styles(element: scraper::ElementRef) -> Option<RnStyles> {
    if let Some(style_attr) = element.value().attr("style") {
        Some(parse_css_declarations(style_attr))
    } else {
        None
    }
}

/// CSS 클래스 스타일 추출
fn extract_class_styles(element: scraper::ElementRef, styles: &HashMap<String, RnStyles>) -> Option<RnStyles> {
    if let Some(class_attr) = element.value().attr("class") {
        for class_name in class_attr.split_whitespace() {
            if let Some(style) = styles.get(class_name) {
                return Some(style.clone());
            }
        }
    }
    None
}

/// 빈 스타일 생성 (헬퍼 함수)
fn create_empty_style() -> RnStyles {
    RnStyles {
        fontSize: None, fontWeight: None, fontFamily: None, color: None,
        backgroundColor: None, textAlign: None, marginTop: None, marginBottom: None,
        marginLeft: None, marginRight: None, paddingTop: None, paddingBottom: None,
        paddingLeft: None, paddingRight: None, lineHeight: None, textDecorationLine: None,
        fontStyle: None,
    }
} 