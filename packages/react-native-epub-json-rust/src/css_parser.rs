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
    let mut styles = HashMap::new();
    
    log!("🎨 전문 CSS 파서로 파싱 시작 (총 {} 바이트)", css.len());
    
    // CSS 내용의 첫 부분을 로그로 출력 (디버깅용)
    if css.len() > 100 {
        log!("   📄 CSS 내용 미리보기: {}", &css[..100]);
    } else if css.len() > 0 {
        log!("   📄 CSS 내용 전체: {}", css);
    }
    
    // CSS 규칙 추출
    let rules = extract_css_rules(css);
    
    log!("   📝 발견된 CSS 규칙: {} 개", rules.len());
    
    let mut parsed_count = 0;
    let mut failed_count = 0;
    
    for (index, (selector, declarations_text)) in rules.iter().enumerate() {
        match parse_css_declarations_with_cssparser(declarations_text) {
            Ok(rn_style) => {
                let style_name = css_selector_to_style_name(selector);
                
                // 첫 5개 규칙은 상세 로그
                if index < 5 {
                    println!("   🔍 규칙 #{}: '{}' → '{}' (✅ 전문 파서)", 
                             index + 1, selector, style_name);
                }
                
                styles.insert(style_name, rn_style);
                parsed_count += 1;
            }
            Err(e) => {
                println!("   ❌ 규칙 #{}: 파싱 실패 - {} (선택자: {})", 
                         index + 1, e, selector);
                failed_count += 1;
            }
        }
    }
    
    let success_rate = if rules.len() > 0 {
        (parsed_count as f32 / rules.len() as f32 * 100.0).round()
    } else {
        100.0
    };
    
    println!("   ✅ 성공적으로 파싱된 스타일: {} 개", parsed_count);
    if failed_count > 0 {
        println!("   ❌ 파싱 실패: {} 개", failed_count);
    }
    println!("   🎯 파싱 성공률: {:.0}% (전문 CSS 파서 사용)", success_rate);
    
    styles
}

/// CSS 규칙을 추출 (선택자 + 선언부)
fn extract_css_rules(css: &str) -> Vec<(String, String)> {
    let mut rules = Vec::new();
    let mut current_rule = String::new();
    let mut brace_count = 0;
    let mut in_rule = false;
    
    log!("   🔍 CSS 규칙 추출 시작...");
    
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
                            // 처음 몇 개 규칙은 로그 출력
                            if rules.len() < 10 {
                                log!("   📝 규칙 #{}: '{}' → '{}'", rules.len() + 1, selector, declarations);
                            }
                            rules.push((selector, declarations));
                        } else {
                            log!("   ⚠️  빈 규칙 건너뜀: selector='{}', declarations='{}'", selector, declarations);
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
    
    log!("   ✅ CSS 규칙 추출 완료: {} 개 규칙 발견", rules.len());
    
    rules
}

/// cssparser를 사용한 전문 CSS 선언 파싱 (공개 함수)
pub fn parse_css_declarations_with_cssparser(declarations: &str) -> Result<RnStyles, String> {
    let mut style = RnStyles {
        fontSize: None, fontWeight: None, fontFamily: None, color: None,
        backgroundColor: None, textAlign: None, marginTop: None, marginBottom: None,
        marginLeft: None, marginRight: None, paddingTop: None, paddingBottom: None,
        paddingLeft: None, paddingRight: None, lineHeight: None, textDecorationLine: None,
        fontStyle: None,
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
    
    if property_count > 3 {
        println!("     💎 리치 스타일 발견: {} 개 속성 변환됨 (전문 파서)", property_count);
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
        "font-size" => style.fontSize = parse_size_value(value),
        "font-weight" => style.fontWeight = Some(value.to_string()),
        "font-family" => style.fontFamily = Some(value.trim_matches('"').to_string()),
        "color" => style.color = Some(value.to_string()),
        "background-color" => style.backgroundColor = Some(value.to_string()),
        "text-align" => style.textAlign = Some(value.to_string()),
        "margin-top" => style.marginTop = parse_size_value(value),
        "margin-bottom" => style.marginBottom = parse_size_value(value),
        "margin-left" => style.marginLeft = parse_size_value(value),
        "margin-right" => style.marginRight = parse_size_value(value),
        "padding-top" => style.paddingTop = parse_size_value(value),
        "padding-bottom" => style.paddingBottom = parse_size_value(value),
        "padding-left" => style.paddingLeft = parse_size_value(value),
        "padding-right" => style.paddingRight = parse_size_value(value),
        "line-height" => style.lineHeight = parse_size_value(value),
        "text-decoration" => {
            if value.contains("underline") {
                style.textDecorationLine = Some("underline".to_string());
            }
        }
        "font-style" => style.fontStyle = Some(value.to_string()),
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

/// 스타일 병합 (인라인 스타일이 클래스 스타일을 덮어씀)
pub fn merge_styles(class_style: Option<RnStyles>, inline_style: Option<RnStyles>) -> Option<RnStyles> {
    match (class_style, inline_style) {
        (Some(mut class), Some(inline)) => {
            // 인라인 스타일이 클래스 스타일을 덮어씀
            if inline.fontSize.is_some() { class.fontSize = inline.fontSize; }
            if inline.fontWeight.is_some() { class.fontWeight = inline.fontWeight; }
            if inline.fontFamily.is_some() { class.fontFamily = inline.fontFamily; }
            if inline.color.is_some() { class.color = inline.color; }
            if inline.backgroundColor.is_some() { class.backgroundColor = inline.backgroundColor; }
            if inline.textAlign.is_some() { class.textAlign = inline.textAlign; }
            if inline.marginTop.is_some() { class.marginTop = inline.marginTop; }
            if inline.marginBottom.is_some() { class.marginBottom = inline.marginBottom; }
            if inline.marginLeft.is_some() { class.marginLeft = inline.marginLeft; }
            if inline.marginRight.is_some() { class.marginRight = inline.marginRight; }
            if inline.paddingTop.is_some() { class.paddingTop = inline.paddingTop; }
            if inline.paddingBottom.is_some() { class.paddingBottom = inline.paddingBottom; }
            if inline.paddingLeft.is_some() { class.paddingLeft = inline.paddingLeft; }
            if inline.paddingRight.is_some() { class.paddingRight = inline.paddingRight; }
            if inline.lineHeight.is_some() { class.lineHeight = inline.lineHeight; }
            if inline.textDecorationLine.is_some() { class.textDecorationLine = inline.textDecorationLine; }
            if inline.fontStyle.is_some() { class.fontStyle = inline.fontStyle; }
            Some(class)
        }
        (Some(class), None) => Some(class),
        (None, Some(inline)) => Some(inline),
        (None, None) => None,
    }
}