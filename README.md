# React Native EPUB JSON Converter

This project is a high-performance EPUB-to-JSON converter designed for **server-driven UI rendering** in React Native. It parses EPUB files into a fully structured JSON format on the **server**, enabling React Native clients to render content using native components (`<View>`, `<Text>`, etc.) without directly processing EPUB files on-device.

## ✅ Purpose

- **Security-first rendering**: EPUB files are parsed on a trusted server, mitigating the risk of content leakage on mobile devices.
- **React Native compatibility**: Output JSON is structured to work seamlessly with native components.
- **Server-driven UI**: Clients only receive the processed JSON, reducing app complexity and runtime cost.

---

## Features

- **Server-side EPUB parsing** using Rust for speed and reliability.
- **WebView-Free Rendering**: No need for WebViews—output is fully compatible with native rendering.
- **Rich EPUB support**: Parses metadata, TOC, spine, HTML content, and CSS styles.
- **Style Conversion**: Converts CSS into React Native-compatible `StyleSheet` objects.
- **Embedded Resources**: Base64-encoded images included in JSON.
- **Modular Output**: Clean JSON structure for easy consumption and rendering on mobile or web.

---

## Project Structure

This is a monorepo organized as follows:

- `packages/react-native-epub-json-rust`:
  Rust-based EPUB parsing engine. Parses EPUB and outputs JSON.

- `packages/react-native-epub-json`:
  WASM wrapper of the Rust parser (used for future client-side rendering or offline processing).

- `examples/v80`:
  Example React Native 0.80.5 project used to test JSON rendering from server.

- `examples/v80-server`:
  Hono-based server that parses EPUB and serves the resulting JSON to clients.

---

## How It Works

1. The server receives an EPUB file.
2. The Rust module parses the EPUB, extracts all relevant content and assets.
3. CSS is converted into a format React Native can render (`StyleSheet`-compatible).
4. A single `book.json` file is generated, containing:

   - Metadata (title, author, etc.)
   - TOC
   - Chapters as renderable node trees
   - Images (base64)
   - Styles

5. The JSON is sent to the client or stored on a CDN for rendering.

---

## Usage (Server)

After setting up the server (e.g., via `v80-server`):

1. POST an EPUB file to the server.
2. Receive a parsed `book.json` in response or access it via CDN.
3. In React Native, consume the JSON and render using native components.

---

## Future Plans

- **Client-side (offline) support** using a Babel-based parser and `EncryptedStorage`
- **Pulumi-based infrastructure setup** for automating server deployment and CDN upload
- **Edge-first delivery**: Pre-parse EPUB files and serve JSON via CDN (e.g., Cloudflare R2 or S3)

---

## Output JSON Structure

```jsonc
{
  "metadata": { /* title, author, publisher, etc. */ },
  "structure": { /* counts for TOC, spine, etc. */ },
  "toc": [ /* table of contents */ ],
  "spine": [ /* linear reading order */ ],
  "styles": { /* converted CSS rules */ },
  "images": { /* base64-encoded images */ },
  "chapters": [
    {
      "type": "View",
      "children": [
        {
          "type": "Text",
          "styles": { "fontWeight": "bold", "lineHeight": 1.20000004768372}
        },
        {
          "type": "Image",
          "src": "data:image/png;base64,...",
          "style": {...}
        }
      ]
    }
  ]
}
```

---

## Building the Parser

To build the WASM module (optional use):

```bash
cd packages/react-native-epub-json-rust
./build-wasm.sh
```

> This will output WASM and JS bindings into `pkg/`.
