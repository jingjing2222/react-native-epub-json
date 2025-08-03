//#region rolldown:runtime
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __commonJS = (cb, mod) => function() {
	return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
};
var __copyProps = (to, from, except, desc) => {
	if (from && typeof from === "object" || typeof from === "function") for (var keys = __getOwnPropNames(from), i = 0, n = keys.length, key; i < n; i++) {
		key = keys[i];
		if (!__hasOwnProp.call(to, key) && key !== except) __defProp(to, key, {
			get: ((k) => from[k]).bind(null, key),
			enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable
		});
	}
	return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", {
	value: mod,
	enumerable: true
}) : target, mod));

//#endregion
const node_fs = __toESM(require("node:fs"));
const node_path = __toESM(require("node:path"));

//#region src/lib/react_native_epub_json.js
var require_react_native_epub_json = /* @__PURE__ */ __commonJS({ "src/lib/react_native_epub_json.js": ((exports, module) => {
	let imports = {};
	imports["__wbindgen_placeholder__"] = module.exports;
	let wasm;
	const { TextEncoder, TextDecoder } = require("util");
	let WASM_VECTOR_LEN = 0;
	let cachedUint8ArrayMemory0 = null;
	function getUint8ArrayMemory0() {
		if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
		return cachedUint8ArrayMemory0;
	}
	let cachedTextEncoder = new TextEncoder("utf-8");
	const encodeString = typeof cachedTextEncoder.encodeInto === "function" ? function(arg, view) {
		return cachedTextEncoder.encodeInto(arg, view);
	} : function(arg, view) {
		const buf = cachedTextEncoder.encode(arg);
		view.set(buf);
		return {
			read: arg.length,
			written: buf.length
		};
	};
	function passStringToWasm0(arg, malloc, realloc) {
		if (realloc === void 0) {
			const buf = cachedTextEncoder.encode(arg);
			const ptr$1 = malloc(buf.length, 1) >>> 0;
			getUint8ArrayMemory0().subarray(ptr$1, ptr$1 + buf.length).set(buf);
			WASM_VECTOR_LEN = buf.length;
			return ptr$1;
		}
		let len = arg.length;
		let ptr = malloc(len, 1) >>> 0;
		const mem = getUint8ArrayMemory0();
		let offset = 0;
		for (; offset < len; offset++) {
			const code = arg.charCodeAt(offset);
			if (code > 127) break;
			mem[ptr + offset] = code;
		}
		if (offset !== len) {
			if (offset !== 0) arg = arg.slice(offset);
			ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
			const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
			const ret = encodeString(arg, view);
			offset += ret.written;
			ptr = realloc(ptr, len, offset, 1) >>> 0;
		}
		WASM_VECTOR_LEN = offset;
		return ptr;
	}
	let cachedDataViewMemory0 = null;
	function getDataViewMemory0() {
		if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || cachedDataViewMemory0.buffer.detached === void 0 && cachedDataViewMemory0.buffer !== wasm.memory.buffer) cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
		return cachedDataViewMemory0;
	}
	let cachedTextDecoder = new TextDecoder("utf-8", {
		ignoreBOM: true,
		fatal: true
	});
	cachedTextDecoder.decode();
	function getStringFromWasm0(ptr, len) {
		ptr = ptr >>> 0;
		return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
	}
	function getArrayU8FromWasm0(ptr, len) {
		ptr = ptr >>> 0;
		return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
	}
	function addToExternrefTable0(obj) {
		const idx = wasm.__externref_table_alloc();
		wasm.__wbindgen_export_5.set(idx, obj);
		return idx;
	}
	function handleError(f, args) {
		try {
			return f.apply(this, args);
		} catch (e) {
			const idx = addToExternrefTable0(e);
			wasm.__wbindgen_exn_store(idx);
		}
	}
	module.exports.main = function() {
		wasm.main();
	};
	function takeFromExternrefTable0(idx) {
		const value = wasm.__wbindgen_export_5.get(idx);
		wasm.__externref_table_dealloc(idx);
		return value;
	}
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
	* @param {string} epub_path
	* @param {string} output_dir
	* @returns {any}
	*/
	module.exports.epubToJson = function(epub_path, output_dir) {
		const ptr0 = passStringToWasm0(epub_path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
		const len0 = WASM_VECTOR_LEN;
		const ptr1 = passStringToWasm0(output_dir, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
		const len1 = WASM_VECTOR_LEN;
		const ret = wasm.epubToJson(ptr0, len0, ptr1, len1);
		if (ret[2]) throw takeFromExternrefTable0(ret[1]);
		return takeFromExternrefTable0(ret[0]);
	};
	function passArray8ToWasm0(arg, malloc) {
		const ptr = malloc(arg.length * 1, 1) >>> 0;
		getUint8ArrayMemory0().set(arg, ptr / 1);
		WASM_VECTOR_LEN = arg.length;
		return ptr;
	}
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
	* @param {Uint8Array} epub_bytes
	* @returns {any}
	*/
	module.exports.epubBytesToJson = function(epub_bytes) {
		const ptr0 = passArray8ToWasm0(epub_bytes, wasm.__wbindgen_malloc);
		const len0 = WASM_VECTOR_LEN;
		const ret = wasm.epubBytesToJson(ptr0, len0);
		if (ret[2]) throw takeFromExternrefTable0(ret[1]);
		return takeFromExternrefTable0(ret[0]);
	};
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
	* @param {string} epub_path
	* @returns {string}
	*/
	module.exports.epubToJsonString = function(epub_path) {
		let deferred3_0;
		let deferred3_1;
		try {
			const ptr0 = passStringToWasm0(epub_path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
			const len0 = WASM_VECTOR_LEN;
			const ret = wasm.epubToJsonString(ptr0, len0);
			var ptr2 = ret[0];
			var len2 = ret[1];
			if (ret[3]) {
				ptr2 = 0;
				len2 = 0;
				throw takeFromExternrefTable0(ret[2]);
			}
			deferred3_0 = ptr2;
			deferred3_1 = len2;
			return getStringFromWasm0(ptr2, len2);
		} finally {
			wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
		}
	};
	module.exports.__wbg_String_8f0eb39a4a4c2f66 = function(arg0, arg1) {
		const ret = String(arg1);
		const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
		const len1 = WASM_VECTOR_LEN;
		getDataViewMemory0().setInt32(arg0 + 4, len1, true);
		getDataViewMemory0().setInt32(arg0 + 0, ptr1, true);
	};
	module.exports.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
		let deferred0_0;
		let deferred0_1;
		try {
			deferred0_0 = arg0;
			deferred0_1 = arg1;
			console.error(getStringFromWasm0(arg0, arg1));
		} finally {
			wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
		}
	};
	module.exports.__wbg_getRandomValues_3c9c0d586e575a16 = function() {
		return handleError(function(arg0, arg1) {
			globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
		}, arguments);
	};
	module.exports.__wbg_log_c222819a41e063d3 = function(arg0) {
		console.log(arg0);
	};
	module.exports.__wbg_new_405e22f390576ce2 = function() {
		const ret = /* @__PURE__ */ new Object();
		return ret;
	};
	module.exports.__wbg_new_5e0be73521bc8c17 = function() {
		const ret = /* @__PURE__ */ new Map();
		return ret;
	};
	module.exports.__wbg_new_78feb108b6472713 = function() {
		const ret = new Array();
		return ret;
	};
	module.exports.__wbg_new_8a6f238a6ece86ea = function() {
		const ret = /* @__PURE__ */ new Error();
		return ret;
	};
	module.exports.__wbg_set_37837023f3d740e8 = function(arg0, arg1, arg2) {
		arg0[arg1 >>> 0] = arg2;
	};
	module.exports.__wbg_set_3f1d0b984ed272ed = function(arg0, arg1, arg2) {
		arg0[arg1] = arg2;
	};
	module.exports.__wbg_set_8fc6bf8a5b1071d1 = function(arg0, arg1, arg2) {
		const ret = arg0.set(arg1, arg2);
		return ret;
	};
	module.exports.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
		const ret = arg1.stack;
		const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
		const len1 = WASM_VECTOR_LEN;
		getDataViewMemory0().setInt32(arg0 + 4, len1, true);
		getDataViewMemory0().setInt32(arg0 + 0, ptr1, true);
	};
	module.exports.__wbindgen_bigint_from_u64 = function(arg0) {
		const ret = BigInt.asUintN(64, arg0);
		return ret;
	};
	module.exports.__wbindgen_error_new = function(arg0, arg1) {
		const ret = new Error(getStringFromWasm0(arg0, arg1));
		return ret;
	};
	module.exports.__wbindgen_init_externref_table = function() {
		const table = wasm.__wbindgen_export_5;
		const offset = table.grow(4);
		table.set(0, void 0);
		table.set(offset + 0, void 0);
		table.set(offset + 1, null);
		table.set(offset + 2, true);
		table.set(offset + 3, false);
	};
	module.exports.__wbindgen_is_string = function(arg0) {
		const ret = typeof arg0 === "string";
		return ret;
	};
	module.exports.__wbindgen_number_new = function(arg0) {
		const ret = arg0;
		return ret;
	};
	module.exports.__wbindgen_string_new = function(arg0, arg1) {
		const ret = getStringFromWasm0(arg0, arg1);
		return ret;
	};
	module.exports.__wbindgen_throw = function(arg0, arg1) {
		throw new Error(getStringFromWasm0(arg0, arg1));
	};
	const path = require("path").join(__dirname, "react_native_epub_json_bg.wasm");
	const bytes = require("fs").readFileSync(path);
	const wasmModule$1 = new WebAssembly.Module(bytes);
	const wasmInstance = new WebAssembly.Instance(wasmModule$1, imports);
	wasm = wasmInstance.exports;
	module.exports.__wasm = wasm;
	wasm.__wbindgen_start();
}) });

//#endregion
//#region src/index.ts
var import_react_native_epub_json = /* @__PURE__ */ __toESM(require_react_native_epub_json());
/**
* Converts an EPUB file to a JSON object and saves it to a file.
* @param epub_path Path to the EPUB file.
* @param output_dir Directory to save the output JSON file.
* @returns The converted JSON object.
*/
function epubToJson(epub_path, output_dir) {
	try {
		const fileBuffer = node_fs.readFileSync(epub_path);
		const result = import_react_native_epub_json.epubBytesToJson(new Uint8Array(fileBuffer));
		if (!node_fs.existsSync(output_dir)) node_fs.mkdirSync(output_dir, { recursive: true });
		const outputPath = node_path.join(output_dir, "book.json");
		const jsonString = JSON.stringify(result, null, 2);
		node_fs.writeFileSync(outputPath, jsonString);
		return result;
	} catch (error) {
		if (error instanceof Error) throw new Error(`EPUB conversion failed: ${error.message}`);
		throw new Error(`EPUB conversion failed: ${String(error)}`);
	}
}
/**
* Converts an EPUB file to a JSON string.
* @param epub_path Path to the EPUB file.
* @returns The JSON string.
*/
function epubToJsonString(epub_path) {
	try {
		const fileBuffer = node_fs.readFileSync(epub_path);
		const result = import_react_native_epub_json.epubBytesToJson(new Uint8Array(fileBuffer));
		return JSON.stringify(result, null, 2);
	} catch (error) {
		if (error instanceof Error) throw new Error(`EPUB conversion failed: ${error.message}`);
		throw new Error(`EPUB conversion failed: ${String(error)}`);
	}
}
/**
* Converts EPUB bytes from memory into a JSON object.
* @param epub_bytes The EPUB file content as a byte array.
* @returns The converted JSON object.
*/
function epubBytesToJson(epub_bytes) {
	return import_react_native_epub_json.epubBytesToJson(epub_bytes);
}
/**
* The raw WASM module for advanced use.
*/
const wasmModule = import_react_native_epub_json;

//#endregion
exports.epubBytesToJson = epubBytesToJson;
exports.epubToJson = epubToJson;
exports.epubToJsonString = epubToJsonString;
exports.wasmModule = wasmModule;
//# sourceMappingURL=index.js.map