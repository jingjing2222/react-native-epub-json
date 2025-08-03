use std::collections::HashMap;
use scraper::{Html, Selector};
use crate::types::{RnNode, RnStyles};
use crate::css_parser::{merge_styles};

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

/// 인라인 CSS 선언을 파싱하는 래퍼 함수
fn parse_css_declarations(declarations: &str) -> RnStyles {
    match crate::css_parser::parse_css_declarations_with_cssparser(declarations) {
        Ok(style) => style,
        Err(_) => {
            // 폴백: 간단한 파싱
            let mut style = RnStyles {
                fontSize: None, fontWeight: None, fontFamily: None, color: None,
                backgroundColor: None, textAlign: None, marginTop: None, marginBottom: None,
                marginLeft: None, marginRight: None, paddingTop: None, paddingBottom: None,
                paddingLeft: None, paddingRight: None, lineHeight: None, textDecorationLine: None,
                fontStyle: None,
            };
            
            for declaration in declarations.split(';') {
                let parts: Vec<&str> = declaration.split(':').collect();
                if parts.len() == 2 {
                    let property = parts[0].trim();
                    let value = parts[1].trim();
                    apply_simple_css_property(&mut style, property, value);
                }
            }
            
            style
        }
    }
}

/// 간단한 CSS 속성 적용 (폴백용)
fn apply_simple_css_property(style: &mut RnStyles, property: &str, value: &str) {
    match property {
        "font-size" => style.fontSize = crate::css_parser::parse_size_value(value),
        "font-weight" => style.fontWeight = Some(value.to_string()),
        "font-family" => style.fontFamily = Some(value.trim_matches('"').to_string()),
        "color" => style.color = Some(value.to_string()),
        "background-color" => style.backgroundColor = Some(value.to_string()),
        "text-align" => style.textAlign = Some(value.to_string()),
        "margin-top" => style.marginTop = crate::css_parser::parse_size_value(value),
        "margin-bottom" => style.marginBottom = crate::css_parser::parse_size_value(value),
        "margin-left" => style.marginLeft = crate::css_parser::parse_size_value(value),
        "margin-right" => style.marginRight = crate::css_parser::parse_size_value(value),
        "padding-top" => style.paddingTop = crate::css_parser::parse_size_value(value),
        "padding-bottom" => style.paddingBottom = crate::css_parser::parse_size_value(value),
        "padding-left" => style.paddingLeft = crate::css_parser::parse_size_value(value),
        "padding-right" => style.paddingRight = crate::css_parser::parse_size_value(value),
        "line-height" => style.lineHeight = crate::css_parser::parse_size_value(value),
        "text-decoration" => {
            if value.contains("underline") {
                style.textDecorationLine = Some("underline".to_string());
            }
        }
        "font-style" => style.fontStyle = Some(value.to_string()),
        _ => {}
    }
}

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
        "p" => {
            // 문단은 기본 상하 마진 추가
            let mut paragraph_style = merged_style.unwrap_or_else(|| create_empty_style());
            if paragraph_style.marginTop.is_none() {
                paragraph_style.marginTop = Some(8.0);
            }
            if paragraph_style.marginBottom.is_none() {
                paragraph_style.marginBottom = Some(8.0);
            }
            RnNode::View { children, styles: Some(paragraph_style) }
        }
        "div" | "section" | "article" | "header" | "footer" => {
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
            
            let styled_children = apply_text_style_to_children(children, &heading_style);
            
            // 헤딩은 일반적으로 블록 요소이므로 항상 View로 감싸기
            RnNode::View { children: styled_children, styles: extract_layout_styles(&heading_style) }
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
            
            // 텍스트 스타일은 항상 Text 노드에 적용
            if children.len() == 1 {
                if let RnNode::Text { content, .. } = &children[0] {
                    return RnNode::Text { content: content.clone(), styles: Some(bold_style) };
                }
            }
            
            // 자식들의 텍스트 노드에 스타일 적용
            let styled_children = apply_text_style_to_children(children, &bold_style);
            
            // 텍스트 스타일만 있으면 View 래핑 없이 텍스트만 반환
            if styled_children.len() == 1 && matches!(styled_children[0], RnNode::Text { .. }) {
                return styled_children.into_iter().next().unwrap();
            }
            
            RnNode::View { children: styled_children, styles: extract_layout_styles(&bold_style) }
        }
        "em" | "i" => {
            let mut italic_style = merged_style.unwrap_or_else(|| create_empty_style());
            italic_style.fontStyle = Some("italic".to_string());
            
            if children.len() == 1 {
                if let RnNode::Text { content, .. } = &children[0] {
                    return RnNode::Text { content: content.clone(), styles: Some(italic_style) };
                }
            }
            
            let styled_children = apply_text_style_to_children(children, &italic_style);
            if styled_children.len() == 1 && matches!(styled_children[0], RnNode::Text { .. }) {
                return styled_children.into_iter().next().unwrap();
            }
            
            RnNode::View { children: styled_children, styles: extract_layout_styles(&italic_style) }
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
        "blockquote" => {
            let mut quote_style = merged_style.unwrap_or_else(|| create_empty_style());
            quote_style.marginLeft = Some(16.0);
            quote_style.marginRight = Some(16.0);
            quote_style.marginTop = Some(8.0);
            quote_style.marginBottom = Some(8.0);
            quote_style.fontStyle = Some("italic".to_string());
            RnNode::View { children, styles: Some(quote_style) }
        }
        "cite" => {
            let mut cite_style = merged_style.unwrap_or_else(|| create_empty_style());
            cite_style.fontStyle = Some("italic".to_string());
            
            if children.len() == 1 {
                if let RnNode::Text { content, .. } = &children[0] {
                    return RnNode::Text { content: content.clone(), styles: Some(cite_style) };
                }
            }
            RnNode::View { children, styles: Some(cite_style) }
        }
        "code" | "tt" => {
            let mut code_style = merged_style.unwrap_or_else(|| create_empty_style());
            code_style.fontFamily = Some("monospace".to_string());
            code_style.fontSize = Some(14.0);
            
            if children.len() == 1 {
                if let RnNode::Text { content, .. } = &children[0] {
                    return RnNode::Text { content: content.clone(), styles: Some(code_style) };
                }
            }
            RnNode::View { children, styles: Some(code_style) }
        }
        "pre" => {
            let mut pre_style = merged_style.unwrap_or_else(|| create_empty_style());
            pre_style.fontFamily = Some("monospace".to_string());
            pre_style.fontSize = Some(14.0);
            pre_style.marginTop = Some(8.0);
            pre_style.marginBottom = Some(8.0);
            RnNode::View { children, styles: Some(pre_style) }
        }
        "sup" => {
            let mut sup_style = merged_style.unwrap_or_else(|| create_empty_style());
            sup_style.fontSize = Some(12.0);
            
            if children.len() == 1 {
                if let RnNode::Text { content, .. } = &children[0] {
                    return RnNode::Text { content: content.clone(), styles: Some(sup_style) };
                }
            }
            RnNode::View { children, styles: Some(sup_style) }
        }
        "sub" => {
            let mut sub_style = merged_style.unwrap_or_else(|| create_empty_style());
            sub_style.fontSize = Some(12.0);
            
            if children.len() == 1 {
                if let RnNode::Text { content, .. } = &children[0] {
                    return RnNode::Text { content: content.clone(), styles: Some(sub_style) };
                }
            }
            RnNode::View { children, styles: Some(sub_style) }
        }
        "small" => {
            let mut small_style = merged_style.unwrap_or_else(|| create_empty_style());
            small_style.fontSize = Some(12.0);
            
            if children.len() == 1 {
                if let RnNode::Text { content, .. } = &children[0] {
                    return RnNode::Text { content: content.clone(), styles: Some(small_style) };
                }
            }
            RnNode::View { children, styles: Some(small_style) }
        }
        "big" => {
            let mut big_style = merged_style.unwrap_or_else(|| create_empty_style());
            big_style.fontSize = Some(20.0);
            
            if children.len() == 1 {
                if let RnNode::Text { content, .. } = &children[0] {
                    return RnNode::Text { content: content.clone(), styles: Some(big_style) };
                }
            }
            RnNode::View { children, styles: Some(big_style) }
        }
        "center" => {
            let mut center_style = merged_style.unwrap_or_else(|| create_empty_style());
            center_style.textAlign = Some("center".to_string());
            RnNode::View { children, styles: Some(center_style) }
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
        log!("   🔍 클래스 속성 발견: '{}'", class_attr);
        for class_name in class_attr.split_whitespace() {
            log!("     - 클래스 '{}' 찾는 중...", class_name);
            
            // 1. 정확한 클래스명으로 먼저 찾기
            if let Some(style) = styles.get(class_name) {
                log!("     ✅ 스타일 매칭 성공 (정확): {} → {:?}", class_name, style);
                return Some(style.clone());
            }
            
            // 2. 복합 선택자에서 이 클래스를 포함한 스타일 찾기
            for (key, style) in styles.iter() {
                // "toc_toc-title" → "toc-title"로 매칭
                // "titlepage_copyright" → "copyright"로 매칭
                if key.contains('_') && key.ends_with(class_name) {
                    log!("     ✅ 복합 선택자 매칭 성공: {} ← {} → {:?}", class_name, key, style);
                    return Some(style.clone());
                }
                // "titlepage_copyright,_titlepage_legalnotice_p" 같은 경우도 처리
                if key.contains(class_name) && (key.contains('_') || key.contains(',')) {
                    log!("     ✅ 복잡한 선택자 매칭 성공: {} ← {} → {:?}", class_name, key, style);
                    return Some(style.clone());
                }
            }
            
            // 3. 일반적인 클래스명에 대한 기본 스타일 제공
            if let Some(default_style) = get_default_class_style(class_name) {
                log!("     ✅ 기본 스타일 적용: {} → {:?}", class_name, default_style);
                return Some(default_style);
            }
            
            log!("     ❌ 스타일 없음: {}", class_name);
        }
    }
    None
}

/// 일반적인 클래스명에 대한 기본 스타일 제공
fn get_default_class_style(class_name: &str) -> Option<RnStyles> {
    match class_name {
        "emphasis" => {
            let mut style = create_empty_style();
            style.fontStyle = Some("italic".to_string());
            Some(style)
        }
        "strong" => {
            let mut style = create_empty_style();
            style.fontWeight = Some("bold".to_string());
            Some(style)
        }
        "center" => {
            let mut style = create_empty_style();
            style.textAlign = Some("center".to_string());
            Some(style)
        }
        "left" => {
            let mut style = create_empty_style();
            style.textAlign = Some("left".to_string());
            Some(style)
        }
        "right" => {
            let mut style = create_empty_style();
            style.textAlign = Some("right".to_string());
            Some(style)
        }
        // EPUB 저자/출판 정보 관련
        "author" | "firstname" | "surname" => {
            let mut style = create_empty_style();
            style.fontWeight = Some("normal".to_string());
            Some(style)
        }
        // EPUB 책 구조 관련
        "book" | "chapter" => {
            let mut style = create_empty_style();
            style.marginTop = Some(16.0);
            style.marginBottom = Some(16.0);
            Some(style)
        }
        "dedication" => {
            let mut style = create_empty_style();
            style.fontStyle = Some("italic".to_string());
            style.textAlign = Some("center".to_string());
            style.marginTop = Some(32.0);
            style.marginBottom = Some(32.0);
            Some(style)
        }
        // 링크 스타일
        "link" => {
            let mut style = create_empty_style();
            style.textDecorationLine = Some("underline".to_string());
            Some(style)
        }
        // 기타 일반적인 EPUB 클래스들
        "subtitle" => {
            let mut style = create_empty_style();
            style.fontStyle = Some("italic".to_string());
            style.fontSize = Some(18.0);
            Some(style)
        }
        "quote" | "quotation" => {
            let mut style = create_empty_style();
            style.fontStyle = Some("italic".to_string());
            style.marginLeft = Some(16.0);
            style.marginRight = Some(16.0);
            Some(style)
        }
        "note" | "footnote" => {
            let mut style = create_empty_style();
            style.fontSize = Some(12.0);
            style.marginTop = Some(8.0);
            style.marginBottom = Some(8.0);
            Some(style)
        }
        "sidebar" => {
            let mut style = create_empty_style();
            style.marginLeft = Some(16.0);
            style.marginRight = Some(16.0);
            style.paddingTop = Some(8.0);
            style.paddingBottom = Some(8.0);
            style.paddingLeft = Some(8.0);
            style.paddingRight = Some(8.0);
            Some(style)
        }
        _ => None
    }
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

/// 자식 노드들의 텍스트에 스타일 적용
fn apply_text_style_to_children(children: Vec<RnNode>, text_style: &RnStyles) -> Vec<RnNode> {
    children.into_iter().map(|child| {
        match child {
            RnNode::Text { content, styles } => {
                let mut combined_style = text_style.clone();
                if let Some(existing_styles) = styles {
                    // 기존 스타일을 우선으로 병합
                    if existing_styles.fontSize.is_some() { combined_style.fontSize = existing_styles.fontSize; }
                    if existing_styles.fontWeight.is_some() { combined_style.fontWeight = existing_styles.fontWeight; }
                    if existing_styles.fontFamily.is_some() { combined_style.fontFamily = existing_styles.fontFamily; }
                    if existing_styles.color.is_some() { combined_style.color = existing_styles.color; }
                    if existing_styles.fontStyle.is_some() { combined_style.fontStyle = existing_styles.fontStyle; }
                    if existing_styles.textDecorationLine.is_some() { combined_style.textDecorationLine = existing_styles.textDecorationLine; }
                    if existing_styles.lineHeight.is_some() { combined_style.lineHeight = existing_styles.lineHeight; }
                }
                RnNode::Text { content, styles: Some(combined_style) }
            }
            RnNode::View { children, styles } => {
                let styled_children = apply_text_style_to_children(children, text_style);
                RnNode::View { children: styled_children, styles }
            }
            other => other
        }
    }).collect()
}

/// 레이아웃 스타일만 추출 (텍스트 스타일 제외)
fn extract_layout_styles(style: &RnStyles) -> Option<RnStyles> {
    let layout_style = RnStyles {
        fontSize: None, fontWeight: None, fontFamily: None, color: None,
        fontStyle: None, textDecorationLine: None, lineHeight: None,
        backgroundColor: style.backgroundColor.clone(),
        textAlign: style.textAlign.clone(),
        marginTop: style.marginTop,
        marginBottom: style.marginBottom,
        marginLeft: style.marginLeft,
        marginRight: style.marginRight,
        paddingTop: style.paddingTop,
        paddingBottom: style.paddingBottom,
        paddingLeft: style.paddingLeft,
        paddingRight: style.paddingRight,
    };
    
    // 레이아웃 스타일이 하나라도 있으면 반환
    if layout_style.backgroundColor.is_some() || layout_style.textAlign.is_some() ||
       layout_style.marginTop.is_some() || layout_style.marginBottom.is_some() ||
       layout_style.marginLeft.is_some() || layout_style.marginRight.is_some() ||
       layout_style.paddingTop.is_some() || layout_style.paddingBottom.is_some() ||
       layout_style.paddingLeft.is_some() || layout_style.paddingRight.is_some() {
        Some(layout_style)
    } else {
        None
    }
} 