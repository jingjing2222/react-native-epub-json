// EPUB to React Native JSON 타입 정의

export interface CompleteEpubInfo {
  metadata: EpubMetadata;
  structure: EpubStructure;
  toc: TocItem[];
  spine: SpineItemInfo[];
  styles: Record<string, RnStyles>;
  images: Record<string, string>;
  chapters: ChapterStructure[];
}

export interface EpubMetadata {
  title?: string;
  author?: string;
  language?: string;
  publisher?: string;
  description?: string;
  date?: string;
  identifier?: string;
  rights?: string;
  subject?: string;
}

export interface EpubStructure {
  spine_count: number;
  resource_count: number;
  toc_count: number;
}

export interface TocItem {
  label: string;
  content_path: string;
}

export interface SpineItemInfo {
  idref: string;
  id?: string;
  properties?: string;
  linear: boolean;
}

export interface RnStyles {
  // 폰트 관련
  fontSize?: number;
  fontWeight?:
    | 'normal'
    | 'bold'
    | '100'
    | '200'
    | '300'
    | '400'
    | '500'
    | '600'
    | '700'
    | '800'
    | '900';
  fontFamily?: string;
  fontStyle?: 'normal' | 'italic';

  // 색상
  color?: string;
  backgroundColor?: string;

  // 텍스트 정렬
  textAlign?: 'auto' | 'left' | 'right' | 'center' | 'justify';
  lineHeight?: number;
  textDecorationLine?:
    | 'none'
    | 'underline'
    | 'line-through'
    | 'underline line-through';

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

  // 레이아웃
  alignSelf?:
    | 'auto'
    | 'flex-start'
    | 'flex-end'
    | 'center'
    | 'stretch'
    | 'baseline';
  width?: number;
  height?: number;
}

export type RnNode = TextNode | ViewNode | ImageNode | ScrollViewNode;

export interface TextNode {
  type: 'Text';
  content: string;
  styles?: RnStyles;
}

export interface ViewNode {
  type: 'View';
  children: RnNode[];
  styles?: RnStyles;
}

export interface ImageNode {
  type: 'Image';
  source: string;
  alt?: string;
  styles?: RnStyles;
}

export interface ScrollViewNode {
  type: 'ScrollView';
  children: RnNode[];
  styles?: RnStyles;
}

export interface ChapterStructure {
  spine_index: number;
  idref: string;
  title?: string;
  content: RnNode;
}
