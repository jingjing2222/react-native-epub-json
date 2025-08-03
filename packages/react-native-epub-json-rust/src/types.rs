use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteEpubInfo {
    pub metadata: EpubMetadata,
    pub structure: EpubStructure,
    pub toc: Vec<TocItem>,
    pub spine: Vec<SpineItemInfo>,
    pub styles: HashMap<String, RnStyles>, // CSS를 RN 스타일로 변환
    pub images: HashMap<String, String>,   // 이미지 ID -> base64 data URI
    pub chapters: Vec<ChapterStructure>,   // HTML을 RN 노드 구조로 변환
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EpubMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub language: Option<String>,
    pub publisher: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
    pub identifier: Option<String>,
    pub rights: Option<String>,
    pub subject: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EpubStructure {
    pub spine_count: usize,
    pub resource_count: usize,
    pub toc_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TocItem {
    pub label: String,
    pub content_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpineItemInfo {
    pub idref: String,
    pub id: Option<String>,
    pub properties: Option<String>,
    pub linear: bool,
}

// React Native 스타일 구조
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct RnStyles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fontSize: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fontWeight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fontFamily: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backgroundColor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textAlign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marginTop: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marginBottom: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marginLeft: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marginRight: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddingTop: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddingBottom: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddingLeft: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paddingRight: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineHeight: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textDecorationLine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fontStyle: Option<String>,
}

// React Native 컴포넌트 노드 구조
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum RnNode {
    Text { 
        content: String, 
        styles: Option<RnStyles> 
    },
    View { 
        children: Vec<RnNode>, 
        styles: Option<RnStyles> 
    },
    Image { 
        source: String,  // base64 data URI
        alt: Option<String>,
        styles: Option<RnStyles> 
    },
    ScrollView { 
        children: Vec<RnNode>, 
        styles: Option<RnStyles> 
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChapterStructure {
    pub spine_index: usize,
    pub idref: String,
    pub title: Option<String>,
    pub content: RnNode, // HTML을 RN 노드 구조로 변환
} 