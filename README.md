# React Native EPUB JSON Converter

This project is a high-performance toolkit for converting EPUB files into a structured JSON format, specifically designed for rendering in React Native applications without needing a WebView. The core logic is written in Rust and compiled to WebAssembly (WASM) for performance and portability.

## Features

- **WebView-Free Rendering:** The output JSON is structured to be rendered directly with native React Native components (`<View>`, `<Text>`, etc.).
- **Comprehensive Parsing:** Parses EPUB metadata, table of contents, spine, HTML content, and CSS styles.
- **Style Conversion:** Automatically converts CSS rules into React Native-compatible StyleSheet objects.
- **Embedded Resources:** Images are extracted and embedded directly into the JSON as base64 data URIs.
- **High Performance:** Core processing is handled by Rust and WASM, making it fast and efficient.
- **Node.js Compatible:** Provides a simple JavaScript wrapper for easy integration into Node.js environments.

## Project Structure

The project is organized as a monorepo with two main packages:

- `packages/react-native-epub-json-rust`: Contains the core Rust source code for the EPUB parsing and conversion logic. This is where all development happens.
- `packages/react-native-epub-json`: The distributable NPM package. The build artifacts from `react-native-epub-json-rust` are placed here. This is the package you would publish or consume in another project.

## How It Works

1.  An EPUB file is read either from the filesystem (in Node.js) or as a byte array.
2.  The Rust/WASM module parses the EPUB container to extract metadata, chapters, images, and CSS files.
3.  The `scraper` library parses the HTML content of each chapter.
4.  The `cssparser` library parses the CSS files.
5.  The HTML structure is transformed into a tree of virtual "React Native" nodes (e.g., `View`, `Text`, `Image`).
6.  CSS rules are converted into JSON objects compatible with React Native's `StyleSheet` and linked to the corresponding nodes.
7.  The final result is a single `book.json` file containing the entire structured content of the EPUB.

## Usage

To use the compiled package in a Node.js project:

```javascript
// Make sure you have the 'react-native-epub-json' package available
const { epubToJson } = require('react-native-epub-json');

// Define the path to your EPUB file and the output directory
const epubPath = 'path/to/your/book.epub';
const outputDir = './output';

try {
  // Convert the EPUB to JSON
  const bookData = epubToJson(epubPath, outputDir);

  // The JSON is also saved to 'output/book.json'
  console.log('Successfully converted EPUB!');
  console.log('Book Title:', bookData.metadata.title);
} catch (error) {
  console.error('Failed to convert EPUB:', error);
}
```

## Building from Source

To build the WASM package from the Rust source code, navigate to the `packages/react-native-epub-json-rust` directory and run the build script.

**Prerequisites:**
- Rust toolchain
- `wasm-pack`

```bash
cd packages/react-native-epub-json-rust
./build-wasm.sh
```

This script will compile the Rust code to WASM, create the necessary JavaScript bindings, and place the final package contents into the `packages/react-native-epub-json-rust/pkg` directory. You would then typically copy these files to the `packages/react-native-epub-json` directory for distribution.

## Output JSON Structure

The generated `book.json` has the following top-level structure:

- `metadata`: Contains book metadata like title, author, publisher, etc.
- `structure`: Provides counts of spine items, resources, and TOC entries.
- `toc`: The table of contents, as an ordered list of items.
- `spine`: A list of all content documents in their linear reading order.
- `styles`: A map of all parsed CSS rules, converted to React Native style objects.
- `images`: A map of all images, with keys as resource IDs and values as base64 data URIs.
- `chapters`: An array containing the content of each chapter, structured as a tree of renderable nodes (`View`, `Text`, `Image`).
