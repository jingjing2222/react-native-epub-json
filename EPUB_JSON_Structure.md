# EPUB JSON 구조 및 React Native 렌더링 가이드

이 문서는 Rust에서 HTML을 완전히 파싱하여 React Native 컴포넌트 구조로 변환한 JSON의 구조와 사용법을 설명합니다.

**핵심 개념**: Rust에서 모든 파싱(HTML, CSS, 이미지)을 완료하고, React Native는 구조화된 데이터만 렌더링합니다. **웹뷰 필요 없음!**

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

// React Native 스타일 타입 (Rust에서 변환됨)
interface RnStyles {
  fontSize?: number;
  fontWeight?: string;
  fontFamily?: string;
  color?: string;
  backgroundColor?: string;
  textAlign?: string;
  marginTop?: number;
  marginBottom?: number;
  marginLeft?: number;
  marginRight?: number;
  paddingTop?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  paddingRight?: number;
  lineHeight?: number;
  textDecorationLine?: string;
  fontStyle?: string;
}

// React Native 노드 구조 (Rust에서 HTML을 변환)
type RnNode =
  | { type: "Text"; content: string; styles?: RnStyles }
  | { type: "View"; children: RnNode[]; styles?: RnStyles }
  | { type: "Image"; source: string; alt?: string; styles?: RnStyles }
  | { type: "ScrollView"; children: RnNode[]; styles?: RnStyles };

// 챕터 구조 타입
interface ChapterStructure {
  spine_index: number;
  idref: string;
  title?: string;
  content: RnNode; // HTML이 이미 RN 노드로 변환됨
}

// 최상위 EPUB 정보 타입
interface CompleteEpubInfo {
  metadata: EpubMetadata;
  structure: EpubStructure;
  toc: TocItem[];
  spine: SpineItemInfo[];
  styles: Record<string, RnStyles>; // CSS → RN 스타일 변환 완료
  images: Record<string, string>; // 이미지 ID → base64 data URI
  chapters: ChapterStructure[]; // HTML → RN 노드 구조 변환 완료
}
```

## React Native에서 렌더링하는 방법

### 1. JSON 로딩

```typescript
import { CompleteEpubInfo } from "./types/epub";

// JSON 파일 로딩
const loadEpubData = async (): Promise<CompleteEpubInfo> => {
  const response = await fetch("path/to/epub_complete.json");
  return response.json();
};
```

### 2. RnNode를 React Native 컴포넌트로 렌더링

**이제 HTML 파싱이 필요 없습니다!** Rust에서 이미 RN 노드 구조로 변환되었기 때문입니다.

```typescript
import React from "react";
import { View, Text, Image, ScrollView, StyleSheet } from "react-native";

// RnNode를 React Native 컴포넌트로 직접 렌더링
const RnNodeRenderer: React.FC<{ node: RnNode }> = ({ node }) => {
  switch (node.type) {
    case "Text":
      return (
        <Text style={node.styles ? convertStyles(node.styles) : undefined}>
          {node.content}
        </Text>
      );

    case "View":
      return (
        <View style={node.styles ? convertStyles(node.styles) : undefined}>
          {node.children.map((child, index) => (
            <RnNodeRenderer key={index} node={child} />
          ))}
        </View>
      );

    case "Image":
      return (
        <Image
          source={{ uri: node.source }} // 이미 base64 data URI로 변환됨
          alt={node.alt}
          style={[
            { width: "100%", height: 200, resizeMode: "contain" },
            node.styles ? convertStyles(node.styles) : undefined,
          ]}
        />
      );

    case "ScrollView":
      return (
        <ScrollView
          style={node.styles ? convertStyles(node.styles) : undefined}
        >
          {node.children.map((child, index) => (
            <RnNodeRenderer key={index} node={child} />
          ))}
        </ScrollView>
      );

    default:
      return null;
  }
};

// Rust의 RnStyles를 React Native StyleSheet로 변환
const convertStyles = (rnStyles: RnStyles) => {
  const style: any = {};

  // 모든 스타일 속성이 이미 React Native 형식으로 변환되어 있음
  Object.entries(rnStyles).forEach(([key, value]) => {
    if (value !== undefined && value !== null) {
      style[key] = value;
    }
  });

  return style;
};
```

### 3. 완전한 EPUB 리더 컴포넌트

```typescript
import React, { useState, useEffect } from "react";
import {
  View,
  ScrollView,
  Text,
  StyleSheet,
  TouchableOpacity,
} from "react-native";

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

      {/* 챕터 내용 - 이미 RN 노드 구조로 변환됨 */}
      <ScrollView style={styles.content}>
        <RnNodeRenderer node={chapter.content} />
      </ScrollView>

      {/* 네비게이션 */}
      <View style={styles.navigation}>
        <TouchableOpacity
          onPress={() => setCurrentChapter(Math.max(0, currentChapter - 1))}
          disabled={currentChapter === 0}
        >
          <Text style={styles.navButton}>이전</Text>
        </TouchableOpacity>

        <Text style={styles.chapterInfo}>
          {currentChapter + 1} / {epubData.chapters.length}
        </Text>

        <TouchableOpacity
          onPress={() =>
            setCurrentChapter(
              Math.min(epubData.chapters.length - 1, currentChapter + 1)
            )
          }
          disabled={currentChapter === epubData.chapters.length - 1}
        >
          <Text style={styles.navButton}>다음</Text>
        </TouchableOpacity>
      </View>
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
    alignItems: "center",
    padding: 20,
    borderTopWidth: 1,
    borderTopColor: "#eee",
  },
  navButton: {
    fontSize: 16,
    color: "#007AFF",
    paddingHorizontal: 20,
    paddingVertical: 10,
  },
  chapterInfo: {
    fontSize: 14,
    color: "#666",
  },
});
```

### 4. 목차(TOC) 컴포넌트

```typescript
const TableOfContents: React.FC<{
  toc: TocItem[];
  chapters: ChapterStructure[];
  onChapterSelect: (chapterIndex: number) => void;
}> = ({ toc, chapters, onChapterSelect }) => {
  return (
    <ScrollView style={styles.tocContainer}>
      <Text style={styles.tocTitle}>목차</Text>
      {toc.map((item, index) => {
        // TOC 항목을 챕터 인덱스와 매칭
        const chapterIndex = chapters.findIndex(
          (chapter) =>
            chapter.title === item.label ||
            chapter.idref.includes(item.content_path)
        );

        return (
          <TouchableOpacity
            key={index}
            style={styles.tocItem}
            onPress={() => chapterIndex >= 0 && onChapterSelect(chapterIndex)}
          >
            <Text style={styles.tocLabel}>{item.label}</Text>
          </TouchableOpacity>
        );
      })}
    </ScrollView>
  );
};
```

### 5. 스타일 커스터마이징

이제 Rust에서 변환된 스타일을 사용자 취향에 맞게 오버라이드할 수 있습니다:

```typescript
const createCustomStyles = (baseStyles: Record<string, RnStyles>) => {
  // 기본 스타일에 사용자 정의 스타일 추가
  const customStyles = {
    ...baseStyles,
    // 사용자 정의 스타일
    readingMode: {
      backgroundColor: "#f5f5dc", // 베이지 배경
      color: "#2f2f2f",
    },
    nightMode: {
      backgroundColor: "#1a1a1a", // 다크 배경
      color: "#e0e0e0",
    },
  };

  return customStyles;
};

// 다크 모드 지원
const DarkModeProvider: React.FC<{
  children: React.ReactNode;
  isDark: boolean;
}> = ({ children, isDark }) => {
  const themeStyle = isDark
    ? { backgroundColor: "#1a1a1a", color: "#e0e0e0" }
    : { backgroundColor: "#ffffff", color: "#000000" };

  return <View style={[styles.container, themeStyle]}>{children}</View>;
};
```

## 주요 장점

### ✅ 성능 최적화

- **Rust에서 파싱 완료**: HTML, CSS, 이미지 모든 처리가 사전 완료
- **React Native는 렌더링만**: 파싱 오버헤드 없음
- **메모리 효율성**: 구조화된 데이터만 로드

### ✅ 완전한 네이티브 경험

- **웹뷰 불필요**: 모든 콘텐츠가 네이티브 컴포넌트
- **플랫폼별 최적화**: iOS/Android 각각 최적화된 렌더링
- **사용자 정의 가능**: 폰트, 색상, 레이아웃 자유자재로 변경

### ✅ 오프라인 지원

- **Self-contained JSON**: 원본 EPUB 파일 불필요
- **임베디드 이미지**: Base64로 인코딩된 모든 이미지
- **스타일 포함**: CSS가 RN StyleSheet로 사전 변환

## 생성된 JSON 구조 예시

```json
{
  "metadata": {
    "title": "The Old Man and the Sea",
    "author": "Ernest Hemingway",
    "language": "en"
  },
  "styles": {
    "h1": {
      "fontSize": 24,
      "fontWeight": "bold",
      "textAlign": "center"
    }
  },
  "images": {
    "cover.jpg": "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQ..."
  },
  "chapters": [
    {
      "spine_index": 0,
      "idref": "chapter1",
      "title": "Chapter 1",
      "content": {
        "type": "View",
        "children": [
          {
            "type": "Text",
            "content": "He was an old man who fished alone...",
            "styles": {
              "fontSize": 16,
              "lineHeight": 24
            }
          }
        ]
      }
    }
  ]
}
```

## 필요한 의존성

React Native 프로젝트에서는 추가 의존성이 필요하지 않습니다! 모든 파싱이 Rust에서 완료되었기 때문입니다.

```bash
# 추가 설치 필요 없음 - React Native 기본 컴포넌트만 사용
# npm install react-native-render-html (불필요)
# npm install htmlparser2 (불필요)
# npm install css-tree (불필요)
```

**🎉 이제 웹뷰 없이 완전히 네이티브 React Native 컴포넌트로 EPUB을 렌더링할 수 있습니다!**
