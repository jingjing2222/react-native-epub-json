# EPUB JSON 구조 및 React Native 파싱 가이드

이 문서는 Rust로 생성된 EPUB JSON의 구조와 React Native에서 웹뷰 없이 네이티브 컴포넌트로 렌더링하는 방법을 설명합니다.

## TypeScript 타입 정의

```typescript
// 메타데이터 타입
interface EpubMetadata {
  title: string | null;
  author: string | null;
  language: string | null;
  publisher: string | null;
  description: string | null;
  date: string | null;
  identifier: string | null;
  rights: string | null;
  subject: string | null;
}

// 구조 정보 타입
interface EpubStructure {
  spine_count: number;
  resource_count: number;
  toc_count: number;
}

// 목차 항목 타입
interface TocItem {
  label: string;
  content_path: string;
}

// Spine 항목 타입
interface SpineItemInfo {
  idref: string;
  id: string | null;
  properties: string | null;
  linear: boolean;
}

// 리소스 데이터 타입 (Union Type)
type ResourceData =
  | { type: "Text"; data: string } // CSS, XHTML 등
  | { type: "Binary"; data: string } // 이미지 등 (base64)
  | { type: "Error"; message: string }; // 읽기 실패

// 리소스 내용 타입
interface ResourceContent {
  path: string;
  mime_type: string;
  content: ResourceData;
}

// 챕터 내용 타입
interface ChapterContent {
  spine_index: number;
  idref: string;
  path: string;
  mime_type: string;
  content: string | null;
  error: string | null;
}

// 최상위 EPUB 정보 타입
interface CompleteEpubInfo {
  metadata: EpubMetadata;
  structure: EpubStructure;
  toc: TocItem[];
  spine: SpineItemInfo[];
  resources: Record<string, ResourceContent>;
  chapters: ChapterContent[];
}
```

## React Native에서 파싱하는 방법

### 1. JSON 로딩

```typescript
import { CompleteEpubInfo } from "./types/epub";

// JSON 파일 로딩
const loadEpubData = async (): Promise<CompleteEpubInfo> => {
  const response = await fetch("path/to/epub_complete.json");
  return response.json();
};
```

### 2. HTML → React Native 컴포넌트 변환

웹뷰 없이 HTML을 RN 컴포넌트로 변환하려면 HTML 파서가 필요합니다:

```bash
npm install react-native-render-html
# 또는
npm install htmlparser2 react-native-super-grid
```

```typescript
import RenderHtml from "react-native-render-html";
import { Dimensions } from "react-native";

const { width } = Dimensions.get("window");

// HTML을 RN 컴포넌트로 렌더링
const EpubChapter: React.FC<{ chapter: ChapterContent }> = ({ chapter }) => {
  if (!chapter.content) return null;

  // HTML에서 body 내용만 추출
  const htmlContent = extractBodyContent(chapter.content);

  return (
    <RenderHtml
      contentWidth={width}
      source={{ html: htmlContent }}
      tagsStyles={getCustomStyles()}
    />
  );
};

// HTML에서 body 태그 내용만 추출
const extractBodyContent = (html: string): string => {
  const bodyMatch = html.match(/<body[^>]*>(.*?)<\/body>/s);
  return bodyMatch ? bodyMatch[1] : html;
};
```

### 3. CSS → StyleSheet 변환

CSS를 React Native StyleSheet으로 변환:

```typescript
import { StyleSheet } from "react-native";

// CSS 리소스에서 스타일 추출
const parseCssStyles = (epubData: CompleteEpubInfo) => {
  const styles: Record<string, any> = {};

  Object.entries(epubData.resources).forEach(([id, resource]) => {
    if (resource.mime_type === "text/css" && resource.content.type === "Text") {
      const cssText = resource.content.data;
      const parsedStyles = parseCssToRnStyles(cssText);
      Object.assign(styles, parsedStyles);
    }
  });

  return StyleSheet.create(styles);
};

// 간단한 CSS → RN 스타일 변환기
const parseCssToRnStyles = (css: string): Record<string, any> => {
  const styles: Record<string, any> = {};

  // CSS 규칙 매칭 (간단한 버전)
  const ruleRegex = /([^{]+){([^}]+)}/g;
  let match;

  while ((match = ruleRegex.exec(css)) !== null) {
    const selector = match[1].trim();
    const declarations = match[2].trim();

    const style = parseDeclarations(declarations);
    styles[selectorToStyleName(selector)] = style;
  }

  return styles;
};

// CSS 선언을 RN 스타일로 변환
const parseDeclarations = (declarations: string): any => {
  const style: any = {};

  declarations.split(";").forEach((decl) => {
    const [property, value] = decl.split(":").map((s) => s.trim());
    if (!property || !value) return;

    const rnProperty = cssPropertyToRn(property);
    const rnValue = cssValueToRn(value);

    if (rnProperty && rnValue !== null) {
      style[rnProperty] = rnValue;
    }
  });

  return style;
};

// CSS 속성을 RN 속성으로 매핑
const cssPropertyToRn = (property: string): string | null => {
  const mapping: Record<string, string> = {
    "font-size": "fontSize",
    "font-weight": "fontWeight",
    "font-family": "fontFamily",
    color: "color",
    "background-color": "backgroundColor",
    "text-align": "textAlign",
    margin: "margin",
    "margin-top": "marginTop",
    "margin-bottom": "marginBottom",
    "margin-left": "marginLeft",
    "margin-right": "marginRight",
    padding: "padding",
    "padding-top": "paddingTop",
    "padding-bottom": "paddingBottom",
    "padding-left": "paddingLeft",
    "padding-right": "paddingRight",
    "line-height": "lineHeight",
  };

  return mapping[property] || null;
};

// CSS 값을 RN 값으로 변환
const cssValueToRn = (value: string): any => {
  // px 단위 제거
  if (value.endsWith("px")) {
    return parseInt(value.replace("px", ""));
  }

  // em을 대략적인 px로 변환 (16px = 1em)
  if (value.endsWith("em")) {
    return parseInt(value.replace("em", "")) * 16;
  }

  // 색상 값
  if (value.startsWith("#") || value.startsWith("rgb")) {
    return value;
  }

  // 폰트 굵기
  if (value === "bold") return "bold";
  if (value === "normal") return "normal";

  // 텍스트 정렬
  if (["left", "center", "right", "justify"].includes(value)) {
    return value;
  }

  return value;
};
```

### 4. 이미지 처리

Base64 이미지를 React Native에서 사용:

```typescript
import { Image } from "react-native";

// Base64 이미지 컴포넌트
const EpubImage: React.FC<{
  resourceId: string;
  epubData: CompleteEpubInfo;
}> = ({ resourceId, epubData }) => {
  const resource = epubData.resources[resourceId];

  if (!resource || resource.content.type !== "Binary") {
    return null;
  }

  const base64Data = resource.content.data;
  const mimeType = resource.mime_type;
  const dataUri = `data:${mimeType};base64,${base64Data}`;

  return (
    <Image
      source={{ uri: dataUri }}
      style={{ width: "100%", height: 200 }}
      resizeMode="contain"
    />
  );
};
```

### 5. 완전한 EPUB 리더 컴포넌트

```typescript
import React, { useState, useEffect } from "react";
import { View, ScrollView, Text, StyleSheet } from "react-native";

const EpubReader: React.FC<{ jsonPath: string }> = ({ jsonPath }) => {
  const [epubData, setEpubData] = useState<CompleteEpubInfo | null>(null);
  const [currentChapter, setCurrentChapter] = useState(0);

  useEffect(() => {
    loadEpubData().then(setEpubData);
  }, []);

  if (!epubData) {
    return <Text>Loading...</Text>;
  }

  const chapter = epubData.chapters[currentChapter];

  return (
    <View style={styles.container}>
      {/* 메타데이터 표시 */}
      <View style={styles.header}>
        <Text style={styles.title}>{epubData.metadata.title}</Text>
        <Text style={styles.author}>{epubData.metadata.author}</Text>
      </View>

      {/* 챕터 내용 */}
      <ScrollView style={styles.content}>
        <EpubChapter chapter={chapter} />
      </ScrollView>

      {/* 네비게이션 */}
      <View style={styles.navigation}>{/* 이전/다음 버튼 등 */}</View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "#fff",
  },
  header: {
    padding: 20,
    borderBottomWidth: 1,
    borderBottomColor: "#eee",
  },
  title: {
    fontSize: 24,
    fontWeight: "bold",
  },
  author: {
    fontSize: 16,
    color: "#666",
    marginTop: 5,
  },
  content: {
    flex: 1,
    padding: 20,
  },
  navigation: {
    flexDirection: "row",
    justifyContent: "space-between",
    padding: 20,
    borderTopWidth: 1,
    borderTopColor: "#eee",
  },
});
```

## 고급 파싱 기법

### 1. 커스텀 HTML 파서

더 정밀한 제어를 원한다면 커스텀 파서 구현:

```typescript
import { XMLParser } from "fast-xml-parser";

const parseHtmlToRnElements = (html: string) => {
  const parser = new XMLParser({
    ignoreAttributes: false,
    attributeNamePrefix: "",
  });

  const parsed = parser.parse(html);
  return convertToRnElements(parsed);
};

const convertToRnElements = (node: any): React.ReactNode => {
  // HTML 노드를 RN 컴포넌트로 재귀적 변환
  // p -> Text, div -> View, img -> Image 등
};
```

### 2. 스타일 상속 처리

CSS 스타일 상속을 RN에서 처리:

```typescript
const applyInheritedStyles = (element: any, parentStyles: any) => {
  // CSS 상속 규칙을 RN에 적용
  return {
    ...parentStyles,
    ...element.styles,
  };
};
```

## 추천 라이브러리

웹뷰 없는 EPUB 렌더링을 위한 유용한 라이브러리들:

```bash
# HTML 렌더링
npm install react-native-render-html

# XML/HTML 파싱
npm install fast-xml-parser

# CSS 파싱
npm install css-tree

# 이미지 처리
npm install react-native-fast-image
```

이제 이 JSON 구조를 사용해서 완전히 네이티브 React Native 컴포넌트로 EPUB을 렌더링할 수 있습니다! 🚀
