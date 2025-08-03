#!/bin/bash

# EPUB to React Native - WASM 빌드 및 Node.js 래퍼 통합 스크립트
# 사용법: ./build-wasm.sh

set -e  # 에러 발생 시 스크립트 중단

echo "🚀 EPUB to React Native WASM 빌드 시작..."
echo "================================================"

# 1. WASM 빌드
echo ""
echo "📦 1단계: wasm-pack으로 WASM 빌드 중..."
echo "RUSTFLAGS=\"--cfg getrandom_backend=\\\"wasm_js\\\"\" wasm-pack build --target nodejs"
RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\"" wasm-pack build --target nodejs

if [ $? -ne 0 ]; then
    echo "❌ WASM 빌드 실패!"
    exit 1
fi

echo "✅ WASM 빌드 완료!"

# 2. JavaScript Wrapper 복사
echo ""
echo "🔧 2단계: JavaScript wrapper 통합 중..."

# wrapper.js를 pkg/index.js로 복사
if [ -f "wrapper.js" ]; then
    cp wrapper.js pkg/index.js
    echo "✅ wrapper.js → pkg/index.js 복사 완료"
else
    echo "❌ wrapper.js 파일을 찾을 수 없습니다!"
    exit 1
fi

# 3. TypeScript 정의 파일 생성
echo ""
echo "📝 3단계: TypeScript 정의 파일 생성 중..."

cat > pkg/index.d.ts << 'EOF'
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
EOF

echo "✅ TypeScript 정의 파일 생성 완료"

# 4. package.json 업데이트
echo ""
echo "📦 4단계: package.json 업데이트 중..."

# package.json의 main과 types 필드 업데이트
if [ -f "pkg/package.json" ]; then
    # 임시 파일을 사용하여 package.json 수정
    jq '.main = "index.js" | .types = "index.d.ts" | .files += ["index.js", "index.d.ts"]' pkg/package.json > pkg/package_temp.json
    
    if [ $? -eq 0 ]; then
        mv pkg/package_temp.json pkg/package.json
        echo "✅ package.json 업데이트 완료"
    else
        # jq가 없는 경우 수동으로 업데이트
        echo "⚠️  jq를 찾을 수 없습니다. 수동으로 package.json을 업데이트합니다..."
        
        # 백업 생성
        cp pkg/package.json pkg/package.json.backup
        
        # sed를 사용하여 main과 types 필드 업데이트
        sed -i.tmp 's/"main": "epub_to_rn.js"/"main": "index.js"/' pkg/package.json
        sed -i.tmp 's/"types": "epub_to_rn.d.ts"/"types": "index.d.ts"/' pkg/package.json
        
        # files 배열에 새 파일들 추가 (간단한 방법)
        if ! grep -q "index.js" pkg/package.json; then
            sed -i.tmp 's/"epub_to_rn.d.ts"/"epub_to_rn.d.ts",\n    "index.js",\n    "index.d.ts"/' pkg/package.json
        fi
        
        # 임시 파일 정리
        rm -f pkg/package.json.tmp
        
        echo "✅ package.json 수동 업데이트 완료"
    fi
else
    echo "❌ pkg/package.json을 찾을 수 없습니다!"
    exit 1
fi

# 5. 빌드 결과 검증
echo ""
echo "🔍 5단계: 빌드 결과 검증 중..."

# 필수 파일들이 존재하는지 확인
required_files=("pkg/index.js" "pkg/index.d.ts" "pkg/epub_to_rn.js" "pkg/epub_to_rn_bg.wasm" "pkg/package.json")

for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file 존재"
    else
        echo "❌ $file 누락!"
        exit 1
    fi
done

# 6. 완료 메시지
echo ""
echo "================================================"
echo "🎉 WASM 빌드 및 Node.js 래퍼 통합 완료!"
echo ""
echo "📦 생성된 패키지: ./pkg/"
echo "📋 사용 가능한 함수들:"
echo "   • epubToJson(epub_path, output_dir)      - 파일 → JSON + 저장"
echo "   • epubToJsonString(epub_path)            - 파일 → JSON 문자열"
echo "   • epubBytesToJson(epub_bytes)            - 바이트 → JSON"
echo ""
echo "🚀 사용법:"
echo "   const { epubToJson } = require('./pkg');"
echo "   const result = epubToJson('book.epub', './output');"
echo ""
echo "📝 TypeScript 지원: index.d.ts 포함"
echo "================================================"