use std::collections::HashMap;
use crate::types::RnStyles;

/// CSS를 React Native 스타일로 변환
pub fn parse_css_to_rn_styles(css: &str) -> HashMap<String, RnStyles> {
    let mut styles = HashMap::new();
    
    println!("🎨 CSS 파싱 시작 (총 {} 바이트)", css.len());
    
    // CSS를 }로 분할하고 유효한 규칙만 필터링
    let all_rules: Vec<&str> = css.split('}').collect();
    let valid_rules: Vec<&str> = all_rules.iter()
        .map(|rule| rule.trim())
        .filter(|rule| !rule.is_empty() && rule.contains('{'))
        .copied()
        .collect();
    
    println!("   📝 전체 분할된 조각: {} 개", all_rules.len());
    println!("   ✨ 유효한 CSS 규칙: {} 개", valid_rules.len());
    
    let mut parsed_count = 0;
    let mut failed_count = 0;
    
    for (index, rule) in valid_rules.iter().enumerate() {
        if let Some(pos) = rule.find('{') {
            let selector = rule[..pos].trim();
            let declarations = rule[pos + 1..].trim();
            
            if selector.is_empty() {
                println!("   ⚠️  규칙 #{}: 선택자가 비어있음", index + 1);
                failed_count += 1;
            } else if declarations.is_empty() {
                println!("   ⚠️  규칙 #{}: 선언이 비어있음 - 선택자: '{}'", index + 1, selector);
                failed_count += 1;
            } else {
                let rn_style = parse_css_declarations(declarations);
                let style_name = css_selector_to_style_name(selector);
                
                // 첫 5개 규칙은 상세 로그
                if index < 5 {
                    println!("   🔍 규칙 #{}: '{}' → '{}' (선언: {})", 
                             index + 1, selector, style_name, declarations);
                }
                
                styles.insert(style_name, rn_style);
                parsed_count += 1;
            }
        } else {
            // 이 경우는 이제 발생하지 않아야 함 (필터링에서 제외됨)
            println!("   🚨 예상치 못한 오류: '{{'를 찾을 수 없음 - 내용: '{}'", rule);
            failed_count += 1;
        }
    }
    
    let success_rate = if valid_rules.len() > 0 {
        (parsed_count as f32 / valid_rules.len() as f32 * 100.0).round()
    } else {
        100.0
    };
    
    println!("   ✅ 성공적으로 파싱된 스타일: {} 개", parsed_count);
    if failed_count > 0 {
        println!("   ❌ 파싱 실패: {} 개", failed_count);
    }
    println!("   🎯 파싱 성공률: {:.0}%", success_rate);
    
    styles
}

/// CSS 선언을 RN 스타일로 변환
pub fn parse_css_declarations(declarations: &str) -> RnStyles {
    let mut style = RnStyles {
        fontSize: None,
        fontWeight: None,
        fontFamily: None,
        color: None,
        backgroundColor: None,
        textAlign: None,
        marginTop: None,
        marginBottom: None,
        marginLeft: None,
        marginRight: None,
        paddingTop: None,
        paddingBottom: None,
        paddingLeft: None,
        paddingRight: None,
        lineHeight: None,
        textDecorationLine: None,
        fontStyle: None,
    };
    
    let mut property_count = 0;
    
    for declaration in declarations.split(';') {
        let parts: Vec<&str> = declaration.split(':').collect();
        if parts.len() == 2 {
            let property = parts[0].trim();
            let value = parts[1].trim();
            
            match property {
                "font-size" => { style.fontSize = parse_size_value(value); property_count += 1; },
                "font-weight" => { style.fontWeight = Some(value.to_string()); property_count += 1; },
                "font-family" => { style.fontFamily = Some(value.trim_matches('"').to_string()); property_count += 1; },
                "color" => { style.color = Some(value.to_string()); property_count += 1; },
                "background-color" => { style.backgroundColor = Some(value.to_string()); property_count += 1; },
                "text-align" => { style.textAlign = Some(value.to_string()); property_count += 1; },
                "margin-top" => { style.marginTop = parse_size_value(value); property_count += 1; },
                "margin-bottom" => { style.marginBottom = parse_size_value(value); property_count += 1; },
                "margin-left" => { style.marginLeft = parse_size_value(value); property_count += 1; },
                "margin-right" => { style.marginRight = parse_size_value(value); property_count += 1; },
                "padding-top" => { style.paddingTop = parse_size_value(value); property_count += 1; },
                "padding-bottom" => { style.paddingBottom = parse_size_value(value); property_count += 1; },
                "padding-left" => { style.paddingLeft = parse_size_value(value); property_count += 1; },
                "padding-right" => { style.paddingRight = parse_size_value(value); property_count += 1; },
                "line-height" => { style.lineHeight = parse_size_value(value); property_count += 1; },
                "text-decoration" => {
                    if value.contains("underline") {
                        style.textDecorationLine = Some("underline".to_string());
                        property_count += 1;
                    }
                }
                "font-style" => { style.fontStyle = Some(value.to_string()); property_count += 1; },
                _ => {} // 지원하지 않는 속성
            }
        }
    }
    
    // 속성이 많은 경우만 로그 (너무 많은 로그 방지)
    if property_count > 3 {
        println!("     💎 리치 스타일 발견: {} 개 속성 변환됨", property_count);
    }
    
    style
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