use std::collections::HashMap;
use crate::types::RnStyles;

/// CSS를 React Native 스타일로 변환
pub fn parse_css_to_rn_styles(css: &str) -> HashMap<String, RnStyles> {
    let mut styles = HashMap::new();
    
    // 간단한 CSS 파싱 (실제로는 더 복잡한 파서가 필요)
    let rules: Vec<&str> = css.split('}').collect();
    
    for rule in rules {
        if let Some(pos) = rule.find('{') {
            let selector = rule[..pos].trim();
            let declarations = rule[pos + 1..].trim();
            
            if !selector.is_empty() && !declarations.is_empty() {
                let rn_style = parse_css_declarations(declarations);
                let style_name = css_selector_to_style_name(selector);
                styles.insert(style_name, rn_style);
            }
        }
    }
    
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
    
    for declaration in declarations.split(';') {
        let parts: Vec<&str> = declaration.split(':').collect();
        if parts.len() == 2 {
            let property = parts[0].trim();
            let value = parts[1].trim();
            
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
                _ => {}
            }
        }
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