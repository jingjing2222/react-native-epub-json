import * as fs from "fs";
import * as path from "path";
import * as wasm from "./lib/react_native_epub_json";

/**
 * Converts an EPUB file to a JSON object and saves it to a file.
 * @param epub_path Path to the EPUB file.
 * @param output_dir Directory to save the output JSON file.
 * @returns The converted JSON object.
 */
export function epubToJson(epub_path: string, output_dir: string): any {
  try {
    const fileBuffer = fs.readFileSync(epub_path);
    const result = wasm.epubBytesToJson(new Uint8Array(fileBuffer));

    if (!fs.existsSync(output_dir)) {
      fs.mkdirSync(output_dir, { recursive: true });
    }

    const outputPath = path.join(output_dir, "book.json");
    const jsonString = JSON.stringify(result, null, 2);
    fs.writeFileSync(outputPath, jsonString);

    return result;
  } catch (error: any) {
    throw new Error(`EPUB conversion failed: ${error.message}`);
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
    const result = wasm.epubBytesToJson(new Uint8Array(fileBuffer));
    return JSON.stringify(result, null, 2);
  } catch (error: any) {
    throw new Error(`EPUB conversion failed: ${error.message}`);
  }
}

/**
 * Converts EPUB bytes from memory into a JSON object.
 * @param epub_bytes The EPUB file content as a byte array.
 * @returns The converted JSON object.
 */
export function epubBytesToJson(epub_bytes: Uint8Array): any {
  return wasm.epubBytesToJson(epub_bytes);
}

/**
 * The raw WASM module for advanced use.
 */
export const wasmModule = wasm;
