/* tslint:disable */
/* eslint-disable */

/**
 * Converts an EPUB file to a JSON object for use in Node.js (JavaScript wrapper).
 * This is a hybrid approach where JavaScript reads the file and WASM processes it.
 * 
 * # JavaScript Usage
 * ```javascript
 * import { epubToJson } from 'react-native-epub-json';
 * 
 * const result = epubToJson('path/to/book.epub', './output');
 * console.log(result.metadata.title);
 * ```
 */
export function epubToJson(epub_path: string, output_dir: string): any;

/**
 * A simpler version: takes an EPUB path and returns only the JSON string (JavaScript wrapper).
 * This is a hybrid approach where JavaScript reads the file and WASM processes it.
 * 
 * # JavaScript Usage
 * ```javascript
 * import { epubToJsonString } from 'react-native-epub-json';
 * 
 * const jsonString = epubToJsonString('book.epub');
 * const data = JSON.parse(jsonString);
 * ```
 */
export function epubToJsonString(epub_path: string): string;

/**
 * Converts EPUB bytes from memory into a JSON object (pure WASM).
 * 
 * # JavaScript Usage
 * ```javascript
 * import { epubBytesToJson } from 'react-native-epub-json';
 * 
 * const fileBuffer = fs.readFileSync('book.epub');
 * const result = epubBytesToJson(new Uint8Array(fileBuffer));
 * ```
 */
export function epubBytesToJson(epub_bytes: Uint8Array): any;

/**
 * The original WASM module (for advanced users).
 */
export const wasmModule: any;

/**
 * WASM initialization function.
 */
export function main(): void;
