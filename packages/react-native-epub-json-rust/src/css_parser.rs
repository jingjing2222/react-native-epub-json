use std::collections::HashMap;
use cssparser::{Parser, ParserInput, Token};
use crate::types::RnStyles;

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

/// 전문 CSS 파서로 React Native 스타일 변환
pub fn parse_css_to_rn_styles(css: &str) -> HashMap<String, RnStyles> {
    // 기본 EPUB 스타일로 시작
    let mut styles = create_default_epub_styles();
    
    // CSS 규칙 추출
    let rules = extract_css_rules(css);    
    
    let mut parsed_count = 0;
    let mut failed_count = 0;
    
    for (index, (selector, declarations_text)) in rules.iter().enumerate() {
        match parse_css_declarations_with_cssparser(declarations_text) {
            Ok(rn_style) => {
                let style_name = css_selector_to_style_name(selector);
                
                styles.insert(style_name, rn_style);
                parsed_count += 1;
            }
            Err(e) => {
                failed_count += 1;
            }
        }
    }
    
    let success_rate = if rules.len() > 0 {
        (parsed_count as f32 / rules.len() as f32 * 100.0).round()
    } else {
        100.0
    };
    
    styles
}

/// CSS 규칙을 추출 (선택자 + 선언부)
fn extract_css_rules(css: &str) -> Vec<(String, String)> {
    let mut rules = Vec::new();
    let mut current_rule = String::new();
    let mut brace_count = 0;
    let mut in_rule = false;
    
    for ch in css.chars() {
        match ch {
            '{' => {
                brace_count += 1;
                if brace_count == 1 {
                    in_rule = true;
                }
                current_rule.push(ch);
            }
            '}' => {
                brace_count -= 1;
                current_rule.push(ch);
                if brace_count == 0 && in_rule {
                    if let Some(pos) = current_rule.find('{') {
                        let selector = current_rule[..pos].trim().to_string();
                        let declarations = current_rule[pos + 1..current_rule.len() - 1].trim().to_string();
                        if !selector.is_empty() && !declarations.is_empty() {
                            rules.push((selector, declarations));
                        } else {
                        }
                    }
                    current_rule.clear();
                    in_rule = false;
                }
            }
            _ => {
                current_rule.push(ch);
            }
        }
    }
    
    rules
}

/// cssparser를 사용한 전문 CSS 선언 파싱 (공개 함수)
pub fn parse_css_declarations_with_cssparser(declarations: &str) -> Result<RnStyles, String> {
    let mut style = RnStyles {
        // 텍스트 스타일
        fontSize: None,
        fontWeight: None,
        fontFamily: None,
        fontStyle: None,
        color: None,
        textAlign: None,
        textDecorationLine: None,
        textTransform: None,
        lineHeight: None,
        textIndent: None,
        
        // 배경 및 색상
        backgroundColor: None,
        opacity: None,
        
        // 여백
        marginTop: None,
        marginBottom: None,
        marginLeft: None,
        marginRight: None,
        paddingTop: None,
        paddingBottom: None,
        paddingLeft: None,
        paddingRight: None,
        
        // 크기 및 레이아웃
        width: None,
        height: None,
        minWidth: None,
        maxWidth: None,
        minHeight: None,
        maxHeight: None,
        
        // 포지셔닝
        position: None,
        top: None,
        bottom: None,
        left: None,
        right: None,
        zIndex: None,
        
        // Flexbox
        display: None,
        flexDirection: None,
        justifyContent: None,
        alignItems: None,
        alignSelf: None,
        flexWrap: None,
        flex: None,
        flexGrow: None,
        flexShrink: None,
        flexBasis: None,
        
        // 테두리
        borderWidth: None,
        borderTopWidth: None,
        borderBottomWidth: None,
        borderLeftWidth: None,
        borderRightWidth: None,
        borderColor: None,
        borderTopColor: None,
        borderBottomColor: None,
        borderLeftColor: None,
        borderRightColor: None,
        borderRadius: None,
        borderStyle: None,
        
        // 오버플로우
        overflow: None,
    };
    
    let mut property_count = 0;
    
    // 각 선언을 세미콜론으로 분할해서 개별 파싱
    for declaration in declarations.split(';') {
        let declaration = declaration.trim();
        if declaration.is_empty() {
            continue;
        }
        
        // cssparser로 속성과 값 추출
        if let Some((property, value)) = parse_css_declaration_simple(declaration) {
            apply_css_property_to_rn_style(&mut style, &property, &value);
            property_count += 1;
        }
    }
    
    Ok(style)
}

/// 간단한 CSS 선언 파싱 (cssparser 기반)
fn parse_css_declaration_simple(declaration: &str) -> Option<(String, String)> {
    let mut input = ParserInput::new(declaration);
    let mut parser = Parser::new(&mut input);
    
    // 속성명 파싱
    let property = match parser.expect_ident() {
        Ok(ident) => ident.to_string(),
        Err(_) => return None,
    };
    
    // ':' 파싱
    if parser.expect_colon().is_err() {
        return None;
    }
    
    // 값 파싱 - 여러 토큰을 수집
    let mut value_parts = Vec::new();
    while !parser.is_exhausted() {
        match parser.next() {
            Ok(token) => {
                let value_part = match token {
                    Token::Ident(s) => s.to_string(),
                    Token::QuotedString(s) => format!("\"{}\"", s),
                    Token::Number { value, .. } => value.to_string(),
                    Token::Percentage { unit_value, .. } => format!("{}%", unit_value * 100.0),
                    Token::Dimension { value, unit, .. } => format!("{}{}", value, unit),
                    Token::Hash(s) => format!("#{}", s),
                    Token::Function(name) => {
                        // 함수는 간단히 처리
                        format!("{}(...)", name)
                    }
                    _ => continue,
                };
                value_parts.push(value_part);
            }
            Err(_) => break,
        }
    }
    
    if value_parts.is_empty() {
        return None;
    }
    
    let value = value_parts.join(" ");
    Some((property, value))
}

/// CSS 속성을 RN 스타일에 적용
fn apply_css_property_to_rn_style(style: &mut RnStyles, property: &str, value: &str) {
    match property {
        // 텍스트 스타일
        "font-size" => style.fontSize = parse_size_value(value),
        "font-weight" => style.fontWeight = Some(value.to_string()),
        "font-family" => style.fontFamily = Some(value.trim_matches('"').to_string()),
        "font-style" => style.fontStyle = Some(value.to_string()),
        "color" => style.color = Some(value.to_string()),
        "text-align" => style.textAlign = Some(value.to_string()),
        "text-decoration" | "text-decoration-line" => {
            if value.contains("underline") {
                style.textDecorationLine = Some("underline".to_string());
            } else if value.contains("line-through") {
                style.textDecorationLine = Some("line-through".to_string());
            } else if value.contains("none") {
                style.textDecorationLine = Some("none".to_string());
            }
        }
        "text-transform" => style.textTransform = Some(value.to_string()),
        "line-height" => style.lineHeight = parse_size_value(value),
        "text-indent" => style.textIndent = parse_size_value(value),
        
        // 배경 및 색상
        "background-color" => style.backgroundColor = Some(value.to_string()),
        "opacity" => style.opacity = value.parse().ok(),
        
        // 여백
        "margin-top" => style.marginTop = parse_size_value(value),
        "margin-bottom" => style.marginBottom = parse_size_value(value),
        "margin-left" => style.marginLeft = parse_size_value(value),
        "margin-right" => style.marginRight = parse_size_value(value),
        "padding-top" => style.paddingTop = parse_size_value(value),
        "padding-bottom" => style.paddingBottom = parse_size_value(value),
        "padding-left" => style.paddingLeft = parse_size_value(value),
        "padding-right" => style.paddingRight = parse_size_value(value),
        
        // 크기 및 레이아웃
        "width" => style.width = parse_size_value(value),
        "height" => style.height = parse_size_value(value),
        "min-width" => style.minWidth = parse_size_value(value),
        "max-width" => style.maxWidth = parse_size_value(value),
        "min-height" => style.minHeight = parse_size_value(value),
        "max-height" => style.maxHeight = parse_size_value(value),
        
        // 포지셔닝
        "position" => style.position = Some(value.to_string()),
        "top" => style.top = parse_size_value(value),
        "bottom" => style.bottom = parse_size_value(value),
        "left" => style.left = parse_size_value(value),
        "right" => style.right = parse_size_value(value),
        "z-index" => style.zIndex = value.parse().ok(),
        
        // Flexbox
        "display" => style.display = Some(value.to_string()),
        "flex-direction" => style.flexDirection = Some(value.to_string()),
        "justify-content" => style.justifyContent = Some(value.to_string()),
        "align-items" => style.alignItems = Some(value.to_string()),
        "align-self" => style.alignSelf = Some(value.to_string()),
        "flex-wrap" => style.flexWrap = Some(value.to_string()),
        "flex" => style.flex = value.parse().ok(),
        "flex-grow" => style.flexGrow = value.parse().ok(),
        "flex-shrink" => style.flexShrink = value.parse().ok(),
        "flex-basis" => style.flexBasis = parse_size_value(value),
        
        // 테두리
        "border-width" => style.borderWidth = parse_size_value(value),
        "border-top-width" => style.borderTopWidth = parse_size_value(value),
        "border-bottom-width" => style.borderBottomWidth = parse_size_value(value),
        "border-left-width" => style.borderLeftWidth = parse_size_value(value),
        "border-right-width" => style.borderRightWidth = parse_size_value(value),
        "border-color" => style.borderColor = Some(value.to_string()),
        "border-top-color" => style.borderTopColor = Some(value.to_string()),
        "border-bottom-color" => style.borderBottomColor = Some(value.to_string()),
        "border-left-color" => style.borderLeftColor = Some(value.to_string()),
        "border-right-color" => style.borderRightColor = Some(value.to_string()),
        "border-radius" => style.borderRadius = parse_size_value(value),
        "border-style" => style.borderStyle = Some(value.to_string()),
        
        // 오버플로우
        "overflow" => style.overflow = Some(value.to_string()),
        
        _ => {} // 지원하지 않는 속성
    }
}

/// CSS 크기 값을 픽셀 단위로 변환
pub fn parse_size_value(value: &str) -> Option<f32> {
    if value.ends_with("px") {
        value.trim_end_matches("px").parse().ok()
    } else if value.ends_with("em") {
        value.trim_end_matches("em").parse::<f32>().ok().map(|v| v * 16.0)
    } else if value.ends_with("pt") {
        value.trim_end_matches("pt").parse::<f32>().ok().map(|v| v * 1.33)
    } else {
        value.parse().ok()
    }
}

/// CSS 선택자를 스타일 이름으로 변환
pub fn css_selector_to_style_name(selector: &str) -> String {
    selector.trim().replace(".", "").replace("#", "").replace(" ", "_")
}

/// 기본 EPUB 스타일 생성
pub fn create_default_epub_styles() -> HashMap<String, RnStyles> {
    let mut styles = HashMap::new();
    
    // 기본 body 스타일
    styles.insert("body".to_string(), RnStyles {
        fontSize: Some(16.0),
        fontFamily: Some("serif".to_string()),
        color: Some("#000000".to_string()),
        lineHeight: Some(1.5),
        paddingLeft: Some(16.0),
        paddingRight: Some(16.0),
        paddingTop: Some(16.0),
        paddingBottom: Some(16.0),
        ..Default::default()
    });
    
    // h1-h6 헤딩 스타일
    styles.insert("h1".to_string(), RnStyles {
        fontSize: Some(28.0),
        fontWeight: Some("bold".to_string()),
        marginTop: Some(24.0),
        marginBottom: Some(16.0),
        ..Default::default()
    });
    
    styles.insert("h2".to_string(), RnStyles {
        fontSize: Some(24.0),
        fontWeight: Some("bold".to_string()),
        marginTop: Some(20.0),
        marginBottom: Some(14.0),
        ..Default::default()
    });
    
    styles.insert("h3".to_string(), RnStyles {
        fontSize: Some(20.0),
        fontWeight: Some("bold".to_string()),
        marginTop: Some(16.0),
        marginBottom: Some(12.0),
        ..Default::default()
    });
    
    styles.insert("p".to_string(), RnStyles {
        fontSize: Some(16.0),
        marginBottom: Some(12.0),
        textAlign: Some("justify".to_string()),
        ..Default::default()
    });
    
    // 강조 스타일
    styles.insert("em".to_string(), RnStyles {
        fontStyle: Some("italic".to_string()),
        ..Default::default()
    });
    
    styles.insert("strong".to_string(), RnStyles {
        fontWeight: Some("bold".to_string()),
        ..Default::default()
    });
    
    // 인용 스타일
    styles.insert("blockquote".to_string(), RnStyles {
        marginLeft: Some(24.0),
        marginRight: Some(24.0),
        marginTop: Some(16.0),
        marginBottom: Some(16.0),
        paddingLeft: Some(16.0),
        borderLeftWidth: Some(3.0),
        borderLeftColor: Some("#cccccc".to_string()),
        fontStyle: Some("italic".to_string()),
        ..Default::default()
    });
    
    // 중앙 정렬
    styles.insert("center".to_string(), RnStyles {
        textAlign: Some("center".to_string()),
        ..Default::default()
    });
    
    styles
}

/// 기본값이 적용된 RnStyles 생성
impl Default for RnStyles {
    fn default() -> Self {
        RnStyles {
            fontSize: None, fontWeight: None, fontFamily: None, fontStyle: None,
            color: None, textAlign: None, textDecorationLine: None, textTransform: None,
            lineHeight: None, textIndent: None, backgroundColor: None, opacity: None,
            marginTop: None, marginBottom: None, marginLeft: None, marginRight: None,
            paddingTop: None, paddingBottom: None, paddingLeft: None, paddingRight: None,
            width: None, height: None, minWidth: None, maxWidth: None, minHeight: None, maxHeight: None,
            position: None, top: None, bottom: None, left: None, right: None, zIndex: None,
            display: None, flexDirection: None, justifyContent: None, alignItems: None, alignSelf: None,
            flexWrap: None, flex: None, flexGrow: None, flexShrink: None, flexBasis: None,
            borderWidth: None, borderTopWidth: None, borderBottomWidth: None, borderLeftWidth: None, borderRightWidth: None,
            borderColor: None, borderTopColor: None, borderBottomColor: None, borderLeftColor: None, borderRightColor: None,
            borderRadius: None, borderStyle: None, overflow: None,
        }
    }
}

/// 스타일 병합 (인라인 스타일이 클래스 스타일을 덮어씀)
pub fn merge_styles(class_style: Option<RnStyles>, inline_style: Option<RnStyles>) -> Option<RnStyles> {
    match (class_style, inline_style) {
        (Some(mut class), Some(inline)) => {
            // 텍스트 스타일
            if inline.fontSize.is_some() { class.fontSize = inline.fontSize; }
            if inline.fontWeight.is_some() { class.fontWeight = inline.fontWeight; }
            if inline.fontFamily.is_some() { class.fontFamily = inline.fontFamily; }
            if inline.fontStyle.is_some() { class.fontStyle = inline.fontStyle; }
            if inline.color.is_some() { class.color = inline.color; }
            if inline.textAlign.is_some() { class.textAlign = inline.textAlign; }
            if inline.textDecorationLine.is_some() { class.textDecorationLine = inline.textDecorationLine; }
            if inline.textTransform.is_some() { class.textTransform = inline.textTransform; }
            if inline.lineHeight.is_some() { class.lineHeight = inline.lineHeight; }
            if inline.textIndent.is_some() { class.textIndent = inline.textIndent; }
            
            // 배경 및 색상
            if inline.backgroundColor.is_some() { class.backgroundColor = inline.backgroundColor; }
            if inline.opacity.is_some() { class.opacity = inline.opacity; }
            
            // 여백
            if inline.marginTop.is_some() { class.marginTop = inline.marginTop; }
            if inline.marginBottom.is_some() { class.marginBottom = inline.marginBottom; }
            if inline.marginLeft.is_some() { class.marginLeft = inline.marginLeft; }
            if inline.marginRight.is_some() { class.marginRight = inline.marginRight; }
            if inline.paddingTop.is_some() { class.paddingTop = inline.paddingTop; }
            if inline.paddingBottom.is_some() { class.paddingBottom = inline.paddingBottom; }
            if inline.paddingLeft.is_some() { class.paddingLeft = inline.paddingLeft; }
            if inline.paddingRight.is_some() { class.paddingRight = inline.paddingRight; }
            
            // 크기 및 레이아웃
            if inline.width.is_some() { class.width = inline.width; }
            if inline.height.is_some() { class.height = inline.height; }
            if inline.minWidth.is_some() { class.minWidth = inline.minWidth; }
            if inline.maxWidth.is_some() { class.maxWidth = inline.maxWidth; }
            if inline.minHeight.is_some() { class.minHeight = inline.minHeight; }
            if inline.maxHeight.is_some() { class.maxHeight = inline.maxHeight; }
            
            // 포지셔닝
            if inline.position.is_some() { class.position = inline.position; }
            if inline.top.is_some() { class.top = inline.top; }
            if inline.bottom.is_some() { class.bottom = inline.bottom; }
            if inline.left.is_some() { class.left = inline.left; }
            if inline.right.is_some() { class.right = inline.right; }
            if inline.zIndex.is_some() { class.zIndex = inline.zIndex; }
            
            // Flexbox
            if inline.display.is_some() { class.display = inline.display; }
            if inline.flexDirection.is_some() { class.flexDirection = inline.flexDirection; }
            if inline.justifyContent.is_some() { class.justifyContent = inline.justifyContent; }
            if inline.alignItems.is_some() { class.alignItems = inline.alignItems; }
            if inline.alignSelf.is_some() { class.alignSelf = inline.alignSelf; }
            if inline.flexWrap.is_some() { class.flexWrap = inline.flexWrap; }
            if inline.flex.is_some() { class.flex = inline.flex; }
            if inline.flexGrow.is_some() { class.flexGrow = inline.flexGrow; }
            if inline.flexShrink.is_some() { class.flexShrink = inline.flexShrink; }
            if inline.flexBasis.is_some() { class.flexBasis = inline.flexBasis; }
            
            // 테두리
            if inline.borderWidth.is_some() { class.borderWidth = inline.borderWidth; }
            if inline.borderTopWidth.is_some() { class.borderTopWidth = inline.borderTopWidth; }
            if inline.borderBottomWidth.is_some() { class.borderBottomWidth = inline.borderBottomWidth; }
            if inline.borderLeftWidth.is_some() { class.borderLeftWidth = inline.borderLeftWidth; }
            if inline.borderRightWidth.is_some() { class.borderRightWidth = inline.borderRightWidth; }
            if inline.borderColor.is_some() { class.borderColor = inline.borderColor; }
            if inline.borderTopColor.is_some() { class.borderTopColor = inline.borderTopColor; }
            if inline.borderBottomColor.is_some() { class.borderBottomColor = inline.borderBottomColor; }
            if inline.borderLeftColor.is_some() { class.borderLeftColor = inline.borderLeftColor; }
            if inline.borderRightColor.is_some() { class.borderRightColor = inline.borderRightColor; }
            if inline.borderRadius.is_some() { class.borderRadius = inline.borderRadius; }
            if inline.borderStyle.is_some() { class.borderStyle = inline.borderStyle; }
            
            // 오버플로우
            if inline.overflow.is_some() { class.overflow = inline.overflow; }
            
            Some(class)
        }
        (Some(class), None) => Some(class),
        (None, Some(inline)) => Some(inline),
        (None, None) => None,
    }
}