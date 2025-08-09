import * as fs from 'node:fs';
import * as path from 'node:path';
import * as wasm from './lib/react_native_epub_json';

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
  // 텍스트 스타일
  fontSize?: number;
  fontWeight?: string;
  fontFamily?: string;
  fontStyle?: string;
  color?: string;
  textAlign?: string;
  textDecorationLine?: string;
  textTransform?: string;
  lineHeight?: number;
  textIndent?: number;

  // 배경 및 색상
  backgroundColor?: string;
  opacity?: number;

  // 여백
  marginTop?: number;
  marginBottom?: number;
  marginLeft?: number;
  marginRight?: number;
  paddingTop?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  paddingRight?: number;

  // 크기 및 레이아웃
  width?: number;
  height?: number;
  minWidth?: number;
  maxWidth?: number;
  minHeight?: number;
  maxHeight?: number;

  // 포지셔닝
  position?: string;
  top?: number;
  bottom?: number;
  left?: number;
  right?: number;
  zIndex?: number;

  // Flexbox
  display?: string;
  flexDirection?: string;
  justifyContent?: string;
  alignItems?: string;
  alignSelf?: string;
  flexWrap?: string;
  flex?: number;
  flexGrow?: number;
  flexShrink?: number;
  flexBasis?: number;

  // 테두리
  borderWidth?: number;
  borderTopWidth?: number;
  borderBottomWidth?: number;
  borderLeftWidth?: number;
  borderRightWidth?: number;
  borderColor?: string;
  borderTopColor?: string;
  borderBottomColor?: string;
  borderLeftColor?: string;
  borderRightColor?: string;
  borderRadius?: number;
  borderStyle?: string;

  // 오버플로우
  overflow?: string;
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

/**
 * Converts an EPUB file to a JSON object and saves it to a file.
 * @param epub_path Path to the EPUB file.
 * @param output_dir Directory to save the output JSON file.
 * @returns The converted JSON object.
 */
export function epubToJson(
  epub_path: string,
  output_dir: string,
): CompleteEpubInfo {
  try {
    const fileBuffer = fs.readFileSync(epub_path);
    const result: CompleteEpubInfo = wasm.epubBytesToJson(
      new Uint8Array(fileBuffer),
    );

    if (!fs.existsSync(output_dir)) {
      fs.mkdirSync(output_dir, { recursive: true });
    }

    const outputPath = path.join(output_dir, 'book.json');
    const jsonString = JSON.stringify(result, null, 2);
    fs.writeFileSync(outputPath, jsonString);

    return result;
  } catch (error: unknown) {
    if (error instanceof Error) {
      throw new Error(`EPUB conversion failed: ${error.message}`);
    }
    throw new Error(`EPUB conversion failed: ${String(error)}`);
  }
}

/**
 * Converts an EPUB file to a JSON string.
 * @param epub_path Path to the EPUB file.
 * @returns The JSON string.
 */
export function epubToJsonString(epub_path: string): string {
  try {
    const fileBuffer = fs.readFileSync(epub_path);
    const result: CompleteEpubInfo = wasm.epubBytesToJson(
      new Uint8Array(fileBuffer),
    );
    return JSON.stringify(result, null, 2);
  } catch (error: unknown) {
    if (error instanceof Error) {
      throw new Error(`EPUB conversion failed: ${error.message}`);
    }
    throw new Error(`EPUB conversion failed: ${String(error)}`);
  }
}

/**
 * Converts EPUB bytes from memory into a JSON object.
 * @param epub_bytes The EPUB file content as a byte array.
 * @returns The converted JSON object.
 */
export function epubBytesToJson(epub_bytes: Uint8Array): CompleteEpubInfo {
  return wasm.epubBytesToJson(epub_bytes);
}

/**
 * The raw WASM module for advanced use.
 */
export const wasmModule = wasm;
