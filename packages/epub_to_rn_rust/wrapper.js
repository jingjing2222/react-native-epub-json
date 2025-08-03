const fs = require("fs");
const path = require("path");

// WASM 모듈 로드
let wasmModule;
try {
  wasmModule = require("./epub_to_rn.js");
  console.log("🔧 WASM 모듈 로드 성공");
} catch (error) {
  console.error("❌ WASM 모듈 로드 실패:", error.message);
  wasmModule = null;
}

/**
 * Node.js에서 사용할 EPUB → JSON 변환 함수 (파일 저장)
 */
function epubToJson(epub_path, output_dir) {
  if (!wasmModule) {
    throw new Error("WASM 모듈이 로드되지 않았습니다");
  }

  try {
    console.log(`📚 EPUB 파일 읽는 중: ${epub_path}`);
    const fileBuffer = fs.readFileSync(epub_path);

    console.log("🔄 WASM에서 EPUB → JSON 변환 중...");
    const result = wasmModule.epubBytesToJson(new Uint8Array(fileBuffer));

    if (!fs.existsSync(output_dir)) {
      fs.mkdirSync(output_dir, { recursive: true });
    }

    const outputPath = path.join(output_dir, "book.json");
    const jsonString = JSON.stringify(result, null, 2);
    fs.writeFileSync(outputPath, jsonString);

    console.log(`✅ 변환 완료! 저장된 파일: ${outputPath}`);
    return result;
  } catch (error) {
    throw new Error(`EPUB 변환 실패: ${error.message}`);
  }
}

/**
 * EPUB → JSON 문자열 변환 (파일 저장 없이)
 */
function epubToJsonString(epub_path) {
  if (!wasmModule) {
    throw new Error("WASM 모듈이 로드되지 않았습니다");
  }

  try {
    console.log(`📚 EPUB 파일 읽는 중: ${epub_path}`);
    const fileBuffer = fs.readFileSync(epub_path);

    console.log("🔄 WASM에서 EPUB → JSON 변환 중...");
    const result = wasmModule.epubBytesToJson(new Uint8Array(fileBuffer));

    return JSON.stringify(result, null, 2);
  } catch (error) {
    throw new Error(`EPUB 변환 실패: ${error.message}`);
  }
}

/**
 * 메모리에서 직접 EPUB 바이트를 JSON으로 변환
 */
function epubBytesToJson(epub_bytes) {
  if (!wasmModule) {
    throw new Error("WASM 모듈이 로드되지 않았습니다");
  }

  return wasmModule.epubBytesToJson(epub_bytes);
}

// Export
module.exports = {
  epubToJson,
  epubToJsonString,
  epubBytesToJson,

  // 직접 WASM 모듈 접근
  wasmModule: wasmModule,
};
