#!/bin/bash

# React Native EPUB JSON Converter - WASM Build & Node.js Wrapper Integration Script
# Usage: ./build-wasm.sh

set -e  # Exit script on error

echo "🚀 Starting React Native EPUB JSON Converter WASM build..."
echo "================================================"

# 1. Build WASM
echo ""
echo "📦 Step 1: Building WASM with wasm-pack..."
echo "RUSTFLAGS=\"--cfg getrandom_backend=\\\"wasm_js\\\"\" wasm-pack build --target nodejs"
RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\"" wasm-pack build --target nodejs

if [ $? -ne 0 ]; then
    echo "❌ WASM build failed!"
    exit 1
fi

echo "✅ WASM build complete!"

# 2. Copy JavaScript Wrapper
echo ""
echo "🔧 Step 2: Integrating JavaScript wrapper..."

# Copy wrapper.js to pkg/index.js
if [ -f "wrapper.js" ]; then
    cp wrapper.js pkg/index.js
    echo "✅ wrapper.js -> pkg/index.js copied successfully"
else
    echo "❌ wrapper.js not found!"
    exit 1
fi

# 3. Create TypeScript Definition File
echo ""
echo "📝 Step 3: Creating TypeScript definition file..."

cat > pkg/index.d.ts << 'EOF'
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
EOF

echo "✅ TypeScript definition file created successfully"

# 4. Update package.json
echo ""
echo "📦 Step 4: Updating package.json..."

# Update main and types fields in package.json
if [ -f "pkg/package.json" ]; then
    # Use jq to modify package.json if available
    jq '.main = "index.js" | .types = "index.d.ts" | .files += ["index.js", "index.d.ts"]' pkg/package.json > pkg/package_temp.json
    
    if [ $? -eq 0 ]; then
        mv pkg/package_temp.json pkg/package.json
        echo "✅ package.json updated successfully"
    else
        # Fallback to manual update if jq is not found
        echo "⚠️ jq not found. Updating package.json manually..."
        
        # Create a backup
        cp pkg/package.json pkg/package.json.backup
        
        # Use sed to update main and types fields
        sed -i.tmp 's/"main": "react_native_epub_json.js"/"main": "index.js"/' pkg/package.json
        sed -i.tmp 's/"types": "react_native_epub_json.d.ts"/"types": "index.d.ts"/' pkg/package.json
        
        # Add new files to the files array (simple method)
        if ! grep -q "index.js" pkg/package.json; then
            sed -i.tmp 's/"react_native_epub_json.d.ts"/"react_native_epub_json.d.ts",\n    "index.js",\n    "index.d.ts"/' pkg/package.json
        fi
        
        # Clean up temporary files
        rm -f pkg/package.json.tmp
        
        echo "✅ package.json updated manually"
    fi
else
    echo "❌ pkg/package.json not found!"
    exit 1
fi

# 5. Verify Build Artifacts
echo ""
echo "🔍 Step 5: Verifying build artifacts..."

# Check for the existence of required files
required_files=("pkg/index.js" "pkg/index.d.ts" "pkg/react_native_epub_json.js" "pkg/react_native_epub_json_bg.wasm" "pkg/package.json")

for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file exists"
    else
        echo "❌ $file is missing!"
        exit 1
    fi
done

# 6. Clean up unnecessary files
echo ""
echo "🧹 Step 6: Cleaning up unnecessary files..."
rm pkg/react_native_epub_json.d.ts
echo "✅ Removed unnecessary .d.ts file"

# 7. Completion Message
echo ""
echo "================================================"
echo "🎉 WASM build and Node.js wrapper integration complete!"
echo ""
echo "📦 Generated package: ./pkg/"
echo "📋 Available functions:"
echo "   • epubToJson(epub_path, output_dir)      - File -> JSON + Save"
echo "   • epubToJsonString(epub_path)            - File -> JSON String"
echo "   • epubBytesToJson(epub_bytes)            - Bytes -> JSON"
echo ""
echo "🚀 Usage:"
echo "   const { epubToJson } = require('./pkg');"
echo "   const result = epubToJson('book.epub', './output');"
echo ""
echo "📝 TypeScript support included: index.d.ts"
echo "================================================"}