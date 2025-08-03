/* tslint:disable */
/* eslint-disable */
export function main(): void;
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
export function epubToJson(epub_path: string, output_dir: string): any;
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
export function epubBytesToJson(epub_bytes: Uint8Array): any;
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
export function epubToJsonString(epub_path: string): string;
