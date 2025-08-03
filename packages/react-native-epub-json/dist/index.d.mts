declare namespace react_native_epub_json_d_exports {
  export { epubBytesToJson$1 as epubBytesToJson, epubToJson$1 as epubToJson, epubToJsonString$1 as epubToJsonString, main };
}
/* tslint:disable */
/* eslint-disable */
declare function main(): void;
/**
 * Node.js에서 사용할 EPUB → JSON 변환 함수
 * 
 * # JavaScript 사용법
 * ```javascript
 * import { epubToJson } from 'react-native-epub-json';
 * 
 * const result = epubToJson('path/to/book.epub', './output');
 * console.log(result.metadata.title);
 * ```
 */
declare function epubToJson$1(epub_path: string, output_dir: string): any;
/**
 * 메모리에서 EPUB 바이트를 JSON 문자열로 변환 (파일 저장 없이)
 * 
 * # JavaScript 사용법
 * ```javascript
 * import { epubBytesToJson } from 'epub-to-rn';
 * 
 * const fileBuffer = fs.readFileSync('book.epub');
 * const result = epubBytesToJson(new Uint8Array(fileBuffer));
 * ```
 */
declare function epubBytesToJson$1(epub_bytes: Uint8Array): any;
/**
 * 간단한 버전: EPUB 경로를 받아서 JSON 문자열만 반환
 * 
 * # JavaScript 사용법
 * ```javascript
 * import { epubToJsonString } from 'epub-to-rn';
 * 
 * const jsonString = epubToJsonString('book.epub');
 * const data = JSON.parse(jsonString);
 * ```
 */
declare function epubToJsonString$1(epub_path: string): string;
//#endregion
//#region src/index.d.ts
interface CompleteEpubInfo {
  metadata: EpubMetadata;
  structure: EpubStructure;
  toc: TocItem[];
  spine: SpineItemInfo[];
  styles: Record<string, RnStyles>;
  images: Record<string, string>;
  chapters: ChapterStructure[];
}
interface EpubMetadata {
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
interface EpubStructure {
  spine_count: number;
  resource_count: number;
  toc_count: number;
}
interface TocItem {
  label: string;
  content_path: string;
}
interface SpineItemInfo {
  idref: string;
  id?: string;
  properties?: string;
  linear: boolean;
}
interface RnStyles {
  fontSize?: number;
  fontWeight?: string;
  fontFamily?: string;
  fontStyle?: string;
  color?: string;
  backgroundColor?: string;
  textAlign?: string;
  lineHeight?: number;
  textDecorationLine?: string;
  marginTop?: number;
  marginBottom?: number;
  marginLeft?: number;
  marginRight?: number;
  paddingTop?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  paddingRight?: number;
}
type RnNode = TextNode | ViewNode | ImageNode | ScrollViewNode;
interface TextNode {
  type: 'Text';
  content: string;
  styles?: RnStyles;
}
interface ViewNode {
  type: 'View';
  children: RnNode[];
  styles?: RnStyles;
}
interface ImageNode {
  type: 'Image';
  source: string;
  alt?: string;
  styles?: RnStyles;
}
interface ScrollViewNode {
  type: 'ScrollView';
  children: RnNode[];
  styles?: RnStyles;
}
interface ChapterStructure {
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
declare function epubToJson(epub_path: string, output_dir: string): CompleteEpubInfo;
/**
 * Converts an EPUB file to a JSON string.
 * @param epub_path Path to the EPUB file.
 * @returns The JSON string.
 */
declare function epubToJsonString(epub_path: string): string;
/**
 * Converts EPUB bytes from memory into a JSON object.
 * @param epub_bytes The EPUB file content as a byte array.
 * @returns The converted JSON object.
 */
declare function epubBytesToJson(epub_bytes: Uint8Array): CompleteEpubInfo;
/**
 * The raw WASM module for advanced use.
 */
declare const wasmModule: typeof react_native_epub_json_d_exports;
//#endregion
export { ChapterStructure, CompleteEpubInfo, EpubMetadata, EpubStructure, ImageNode, RnNode, RnStyles, ScrollViewNode, SpineItemInfo, TextNode, TocItem, ViewNode, epubBytesToJson, epubToJson, epubToJsonString, wasmModule };
//# sourceMappingURL=index.d.mts.map