const fs = require('fs');
const path = require('path');
const wasmModule = require('./epub_to_rn.js');

/**
 * Node.js에서 사용할 EPUB → JSON 변환 함수
 * JavaScript가 파일을 읽고 WASM이 처리하는 방식
 * 
 * @param {string} epub_path - EPUB 파일 경로
 * @param {string} output_dir - 출력 디렉토리 경로
 * @returns {Object} 변환된 EPUB 정보 객체
 */
function epubToJson(epub_path, output_dir) {
    try {
        // 1. Node.js에서 파일 존재 확인
        if (!fs.existsSync(epub_path)) {
            throw new Error(`EPUB 파일을 찾을 수 없습니다: ${epub_path}`);
        }
        
        // 2. Node.js에서 EPUB 파일 읽기
        console.log(`📚 EPUB 파일 읽는 중: ${epub_path}`);
        const fileBuffer = fs.readFileSync(epub_path);
        
        // 3. WASM에서 바이트 처리
        console.log('🔄 WASM에서 EPUB → JSON 변환 중...');
        const result = wasmModule.epubBytesToJson(new Uint8Array(fileBuffer));
        
        // 4. Node.js에서 출력 디렉토리 생성
        if (!fs.existsSync(output_dir)) {
            console.log(`📁 출력 디렉토리 생성: ${output_dir}`);
            fs.mkdirSync(output_dir, { recursive: true });
        }
        
        // 5. Node.js에서 JSON 파일 저장
        const outputPath = path.join(output_dir, 'book.json');
        const jsonString = JSON.stringify(result, null, 2);
        fs.writeFileSync(outputPath, jsonString);
        
        console.log(`✅ 변환 완료! 저장된 파일: ${outputPath}`);
        return result;
        
    } catch (error) {
        throw new Error(`EPUB 변환 실패: ${error.message}`);
    }
}

/**
 * 간단한 버전: EPUB 경로를 받아서 JSON 문자열만 반환
 * JavaScript가 파일을 읽고 WASM이 처리하는 방식
 * 
 * @param {string} epub_path - EPUB 파일 경로
 * @returns {string} JSON 문자열
 */
function epubToJsonString(epub_path) {
    try {
        // 1. Node.js에서 파일 존재 확인
        if (!fs.existsSync(epub_path)) {
            throw new Error(`EPUB 파일을 찾을 수 없습니다: ${epub_path}`);
        }
        
        // 2. Node.js에서 EPUB 파일 읽기
        console.log(`📚 EPUB 파일 읽는 중: ${epub_path}`);
        const fileBuffer = fs.readFileSync(epub_path);
        
        // 3. WASM에서 바이트 처리
        console.log('🔄 WASM에서 EPUB → JSON 변환 중...');
        const result = wasmModule.epubBytesToJson(new Uint8Array(fileBuffer));
        
        // 4. JSON 문자열로 변환
        return JSON.stringify(result, null, 2);
        
    } catch (error) {
        throw new Error(`EPUB 변환 실패: ${error.message}`);
    }
}

/**
 * 메모리에서 EPUB 바이트를 JSON으로 변환 (기존 함수 그대로)
 * 
 * @param {Uint8Array} epub_bytes - EPUB 파일의 바이트 배열
 * @returns {Object} 변환된 EPUB 정보 객체
 */
function epubBytesToJson(epub_bytes) {
    return wasmModule.epubBytesToJson(epub_bytes);
}

// 기존 WASM 함수들도 export (하위 호환성)
module.exports = {
    // 새로운 JavaScript wrapper 함수들 (권장)
    epubToJson,
    epubToJsonString,
    epubBytesToJson,
    
    // 기존 WASM 함수들 (직접 사용 시 파일 I/O 제한)
    wasmModule,
    
    // 편의 함수
    main: wasmModule.main
};