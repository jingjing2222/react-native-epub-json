# EPUB to React Native JSON 타입 정의

이 문서는 `react-native-epub-json-rust`에서 생성하는 최종 JSON 산출물의 타입 구조를 정의합니다.

## 🔗 최상위 구조체

### `CompleteEpubInfo`

전체 EPUB 데이터를 포함하는 루트 객체입니다.

```typescript
interface CompleteEpubInfo {
  metadata: EpubMetadata; // EPUB 메타데이터
  structure: EpubStructure; // EPUB 구조 정보
  toc: TocItem[]; // 목차 (Table of Contents)
  spine: SpineItemInfo[]; // Spine 순서 정보
  styles: Record<string, RnStyles>; // CSS → RN 스타일 변환 결과
  images: Record<string, string>; // 이미지 ID → base64 data URI
  chapters: ChapterStructure[]; // HTML → RN 노드 구조 변환 결과
}
```

---

## 📖 메타데이터

### `EpubMetadata`

EPUB 파일의 메타데이터 정보입니다.

```typescript
interface EpubMetadata {
  title?: string; // 책 제목
  author?: string; // 저자
  language?: string; // 언어 (예: "ko", "en")
  publisher?: string; // 출판사
  description?: string; // 책 설명
  date?: string; // 출간일
  identifier?: string; // ISBN 등 식별자
  rights?: string; // 저작권 정보
  subject?: string; // 주제/카테고리
}
```

---

## 🏗️ 구조 정보

### `EpubStructure`

EPUB 파일의 구조적 정보입니다.

```typescript
interface EpubStructure {
  spine_count: number; // Spine 항목 개수
  resource_count: number; // 전체 리소스 개수
  toc_count: number; // 목차 항목 개수
}
```

### `TocItem`

목차(Table of Contents) 항목입니다.

```typescript
interface TocItem {
  label: string; // 목차 제목
  content_path: string; // 연결된 콘텐츠 경로
}
```

### `SpineItemInfo`

EPUB의 읽기 순서를 정의하는 Spine 정보입니다.

```typescript
interface SpineItemInfo {
  idref: string; // 참조 ID
  id?: string; // 항목 ID
  properties?: string; // 속성 정보
  linear: boolean; // 선형 읽기 순서 포함 여부
}
```

---

## 🎨 스타일 시스템

### `RnStyles`

CSS에서 React Native 스타일로 변환된 객체입니다.

```typescript
interface RnStyles {
  // 폰트 관련
  fontSize?: number;
  fontWeight?: string; // "normal" | "bold" | "100" ~ "900"
  fontFamily?: string;
  fontStyle?: string; // "normal" | "italic"

  // 색상
  color?: string; // 텍스트 색상
  backgroundColor?: string; // 배경색

  // 텍스트 정렬
  textAlign?: string; // "left" | "center" | "right" | "justify"
  lineHeight?: number;
  textDecorationLine?: string; // "none" | "underline" | "line-through"

  // 여백 (Margin)
  marginTop?: number;
  marginBottom?: number;
  marginLeft?: number;
  marginRight?: number;

  // 안쪽 여백 (Padding)
  paddingTop?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  paddingRight?: number;
}
```

---

## 🧩 컴포넌트 노드

### `RnNode`

HTML에서 React Native 컴포넌트로 변환된 노드 구조입니다.

```typescript
type RnNode = TextNode | ViewNode | ImageNode | ScrollViewNode;

interface TextNode {
  type: "Text";
  content: string; // 텍스트 내용
  styles?: RnStyles; // 적용된 스타일
}

interface ViewNode {
  type: "View";
  children: RnNode[]; // 자식 노드들
  styles?: RnStyles; // 적용된 스타일
}

interface ImageNode {
  type: "Image";
  source: string; // base64 data URI
  alt?: string; // 대체 텍스트
  styles?: RnStyles; // 적용된 스타일
}

interface ScrollViewNode {
  type: "ScrollView";
  children: RnNode[]; // 자식 노드들
  styles?: RnStyles; // 적용된 스타일
}
```

---

## 📚 챕터 구조

### `ChapterStructure`

각 챕터의 구조화된 데이터입니다.

```typescript
interface ChapterStructure {
  spine_index: number; // Spine에서의 순서
  idref: string; // 참조 ID
  title?: string; // 챕터 제목 (HTML에서 추출)
  content: RnNode; // 챕터 내용 (RN 노드 구조)
}
```

---

## 💡 사용 예제

### JavaScript/TypeScript에서 사용

```typescript
import { epubToJson } from "react-native-epub-json";

// EPUB 파일을 JSON으로 변환
const result: CompleteEpubInfo = epubToJson("book.epub", "./output");

// 메타데이터 접근
console.log("제목:", result.metadata.title);
console.log("저자:", result.metadata.author);

// 챕터 렌더링
result.chapters.forEach((chapter, index) => {
  console.log(`챕터 ${index + 1}: ${chapter.title}`);
  // chapter.content는 RnNode 구조로 React Native에서 렌더링 가능
});

// 스타일 사용
Object.entries(result.styles).forEach(([className, style]) => {
  console.log(`스타일 ${className}:`, style);
});

// 이미지 접근
Object.entries(result.images).forEach(([id, dataUri]) => {
  console.log(`이미지 ${id}: ${dataUri.substring(0, 50)}...`);
});
```

### React Native에서 렌더링

```tsx
import React from "react";
import { View, Text, Image, ScrollView } from "react-native";

function renderRnNode(node: RnNode): React.ReactNode {
  switch (node.type) {
    case "Text":
      return <Text style={node.styles}>{node.content}</Text>;

    case "View":
      return (
        <View style={node.styles}>
          {node.children.map((child, index) => (
            <React.Fragment key={index}>{renderRnNode(child)}</React.Fragment>
          ))}
        </View>
      );

    case "Image":
      return (
        <Image
          source={{ uri: node.source }}
          style={node.styles}
          alt={node.alt}
        />
      );

    case "ScrollView":
      return (
        <ScrollView style={node.styles}>
          {node.children.map((child, index) => (
            <React.Fragment key={index}>{renderRnNode(child)}</React.Fragment>
          ))}
        </ScrollView>
      );
  }
}

// 챕터 렌더링 컴포넌트
function ChapterRenderer({ chapter }: { chapter: ChapterStructure }) {
  return (
    <ScrollView>
      {chapter.title && (
        <Text style={{ fontSize: 24, fontWeight: "bold" }}>
          {chapter.title}
        </Text>
      )}
      {renderRnNode(chapter.content)}
    </ScrollView>
  );
}
```

---

## 🔧 주요 특징

### ✅ 완전한 self-contained 구조

- 모든 이미지가 base64 data URI로 임베드됨
- CSS가 React Native 스타일로 완전 변환됨
- 외부 의존성 없이 JSON만으로 렌더링 가능

### ✅ React Native 최적화

- WebView 없이 네이티브 컴포넌트로 렌더링
- React Native StyleSheet와 호환되는 스타일 속성
- 터치/스크롤 등 네이티브 인터랙션 지원

### ✅ 타입 안전성

- TypeScript 타입 정의 제공
- Rust의 강타입 시스템에서 생성
- JSON 스키마 일관성 보장

---

## 🚀 WASM 바인딩

이 라이브러리는 다음 WASM 함수들을 제공합니다:

```typescript
// 파일 경로에서 변환
export function epubToJson(
  epubPath: string,
  outputDir: string
): CompleteEpubInfo;

// 메모리의 바이트에서 변환
export function epubBytesToJson(epubBytes: Uint8Array): CompleteEpubInfo;

// JSON 문자열로 반환
export function epubToJsonString(epubPath: string): string;
```
