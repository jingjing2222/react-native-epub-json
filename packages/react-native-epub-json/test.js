#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");

console.log("🧪 EPUB to React Native WASM 패키지 테스트\n");

// pkg 디렉토리가 존재하는지 확인
if (!fs.existsSync("./dist")) {
  console.log("❌ dist 디렉토리가 없습니다. 먼저 빌드를 실행하세요:");
  console.log("  yarn build");
  process.exit(1);
}

try {
  // 패키지 로드
  const { epubToJson, epubToJsonString, epubBytesToJson } = require("./dist/index.js");

  console.log("✅ 패키지 로드 성공");
  console.log("📋 사용 가능한 함수들:");
  console.log("   • epubToJson:", typeof epubToJson);
  console.log("   • epubToJsonString:", typeof epubToJsonString);
  console.log("   • epubBytesToJson:", typeof epubBytesToJson);

  // 테스트 EPUB 파일 찾기
  const testFiles = ["/Users/kimhyeongjeong/Desktop/code/ebook/hemingway-old-man-and-the-sea.epub"];

  let testFile = null;
  for (const file of testFiles) {
    if (fs.existsSync(file)) {
      testFile = file;
      break;
    }
  }

  if (!testFile) {
    console.log("\n⚠️  테스트용 EPUB 파일을 찾을 수 없습니다.");
    console.log("   다음 중 하나의 파일을 준비해주세요:");
    testFiles.forEach((file) => console.log(`   • ${file}`));
    console.log("\n✅ 패키지 구조는 정상입니다.");
    process.exit(0);
  }

  console.log(`\n🔍 테스트 파일: ${testFile}`);

  // 실제 테스트 실행
  console.log("\n1️⃣ epubBytesToJson 테스트 (순수 WASM)");
  try {
    const fileBuffer = fs.readFileSync(testFile);
    const result1 = epubBytesToJson(new Uint8Array(fileBuffer));
    console.log(`   ✅ 성공 - 제목: "${result1.metadata.title}"`);
    const stylesCount =
      result1.styles instanceof Map
        ? result1.styles.size
        : Object.keys(result1.styles || {}).length;
    console.log(`   📊 챕터: ${result1.chapters.length}개, 스타일: ${stylesCount}개`);
  } catch (error) {
    console.log(`   ❌ 실패: ${error.message}`);
  }

  console.log("\n2️⃣ epubToJsonString 테스트 (JS + WASM)");
  try {
    const jsonString = epubToJsonString(testFile);
    const result2 = JSON.parse(jsonString);
    console.log(`   ✅ 성공 - 제목: "${result2.metadata.title}"`);
    console.log(`   📝 JSON 크기: ${Math.round(jsonString.length / 1024)} KB`);
  } catch (error) {
    console.log(`   ❌ 실패: ${error.message}`);
  }

  console.log("\n3️⃣ epubToJson 테스트 (JS + WASM + 파일 저장)");
  try {
    const outputDir = "./test_output_script";
    const result3 = epubToJson(testFile, outputDir);
    console.log(`   ✅ 성공 - 제목: "${result3.metadata.title}"`);

    const savedFile = path.join(outputDir, "book.json");
    if (fs.existsSync(savedFile)) {
      const fileSize = Math.round(fs.statSync(savedFile).size / 1024);
      console.log(`   💾 파일 저장 성공: ${savedFile} (${fileSize} KB)`);
    } else {
      console.log("   ⚠️  파일 저장 실패");
    }
  } catch (error) {
    console.log(`   ❌ 실패: ${error.message}`);
  }

  console.log("\n🎉 모든 테스트 완료!");
} catch (error) {
  console.log("❌ 패키지 로드 실패:", error.message);
  console.log("\n🔧 해결 방법:");
  console.log("   1. npm run build (빌드 실행)");
  console.log("   2. npm run clean && npm run build (클린 빌드)");
  process.exit(1);
}
