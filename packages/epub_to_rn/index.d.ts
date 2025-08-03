/* tslint:disable */
/* eslint-disable */

/**
 * Node.js에서 사용할 EPUB → JSON 변환 함수 (JavaScript wrapper)
 * JavaScript가 파일을 읽고 WASM이 처리하는 하이브리드 방식
 * 
 * # JavaScript 사용법
 * ```javascript
 * import { epubToJson } from 'epub-to-rn';
 * 
 * const result = epubToJson('path/to/book.epub', './output');
 * console.log(result.metadata.title);
 * ```
 */
export function epubToJson(epub_path: string, output_dir: string): any;

/**
 * 간단한 버전: EPUB 경로를 받아서 JSON 문자열만 반환 (JavaScript wrapper)
 * JavaScript가 파일을 읽고 WASM이 처리하는 하이브리드 방식
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

/**
 * 메모리에서 EPUB 바이트를 JSON으로 변환 (순수 WASM)
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
 * 원본 WASM 모듈 (고급 사용자용)
 */
export const wasmModule: any;

/**
 * WASM 초기화 함수
 */
export function main(): void;
