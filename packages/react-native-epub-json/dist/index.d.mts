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
/**
 * Converts an EPUB file to a JSON object and saves it to a file.
 * @param epub_path Path to the EPUB file.
 * @param output_dir Directory to save the output JSON file.
 * @returns The converted JSON object.
 */
declare function epubToJson(epub_path: string, output_dir: string): any;
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
declare function epubBytesToJson(epub_bytes: Uint8Array): any;
/**
 * The raw WASM module for advanced use.
 */
declare const wasmModule: typeof react_native_epub_json_d_exports;
//#endregion
export { epubBytesToJson, epubToJson, epubToJsonString, wasmModule };
//# sourceMappingURL=index.d.mts.map