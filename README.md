# React Native Server-Driven EPUB

> ⚠️ **DEPRECATED & NOT MAINTAINING** - This project is currently deprecated and not actively maintained. 

This project is a WASM-powered EPUB-to-JSON converter designed for **server-driven UI rendering** in React Native. It parses EPUB files into a fully structured JSON format on the **server**, enabling React Native clients to render content using native components (`<View>`, `<Text>`, etc.) without directly processing EPUB files on-device.

## 🚧 Current Status & Future Plans

This project was developed as a **Proof of Concept (POC)** for server-driven EPUB rendering. Currently:

- ⚠️ **Deprecated**: Not actively maintained
- 📦 **WASM-based**: Built with Rust and WebAssembly for performance
- 🔮 **Future Ready**: Waiting for React Native Node API support

### Why WASM?

This project uses **WebAssembly (WASM)** for EPUB parsing, which will become more valuable when React Native officially supports Node API. Once Node API is available in React Native:

- ✨ Direct WASM integration in React Native apps
- 🚀 Client-side EPUB processing without servers
- 📱 Offline-capable EPUB readers

## 🛠️ For Developers

If you're interested in developing this further:

1. **Current POC**: The server-driven implementation is functional and ready for extension
2. **WASM Module**: The core parsing engine is complete and performant
3. **React Native Components**: Basic rendering components are implemented
4. **Future Integration**: Ready for Node API when it becomes available in RN

Feel free to fork, extend, or build upon this work!

---

## Features

- **Server-side EPUB parsing** using Rust/WASM for speed and reliability
- **WebView-Free Rendering**: Output is fully compatible with native React Native components
- **Rich EPUB support**: Parses metadata, TOC, spine, HTML content, and CSS styles
- **Style Conversion**: Converts CSS into React Native-compatible `StyleSheet` objects
- **Embedded Resources**: Base64-encoded images included in JSON
- **Modular Output**: Clean JSON structure for easy consumption and rendering

---

## Project Structure

This is a monorepo organized as follows:

- `packages/react-native-epub-json-rust`:
  Rust-based EPUB parsing engine with WASM bindings

- `packages/react-native-server-driven-epub`:
  Published NPM package with TypeScript bindings for server-side usage

- `packages/react-native-epub-builder`:
  React Native UI components for rendering parsed EPUB content

- `examples/v80`:
  Example React Native 0.80.5 project demonstrating JSON rendering

- `examples/v80-server`:
  Hono-based server that parses EPUB and serves JSON to clients

---

## How It Works

1. **Server receives** an EPUB file
2. **WASM module parses** the EPUB, extracts content and assets
3. **CSS is converted** into React Native-compatible styles
4. **JSON is generated** containing:
   - Metadata (title, author, etc.)
   - Table of Contents
   - Chapters as renderable component trees
   - Images (base64 encoded)
   - Converted styles

5. **Client consumes** the JSON and renders using native RN components

---

## Usage Example

### Server-side (Current Implementation)

```typescript
import { epubBytesToJson } from 'react-native-server-driven-epub';

// Parse EPUB to JSON
const fileBuffer = fs.readFileSync('./book.epub');
const epubData = epubBytesToJson(new Uint8Array(fileBuffer));

// Serve to React Native client
app.get('/epub/:id', (c) => c.json(epubData));
```

### React Native Client

```tsx
import { renderEpubNode } from './EpubRenderer';

const EpubReader = ({ epubData }) => (
  <ScrollView>
    {epubData.chapters.map(chapter => 
      renderEpubNode(chapter.content)
    )}
  </ScrollView>
);
```

---

## Future Integration (Node API)

Once React Native supports Node API, this WASM module can be integrated directly:

```typescript
// Future client-side usage (when Node API is available)
import { parseEpub } from 'react-native-server-driven-epub/wasm';

const epubData = await parseEpub(epubBuffer);
// Render directly in RN without server
```

---

## Output JSON Structure

```jsonc
{
  "metadata": { 
    "title": "Book Title",
    "author": "Author Name",
    // ... other metadata
  },
  "toc": [ /* table of contents */ ],
  "spine": [ /* reading order */ ],
  "styles": { /* RN-compatible styles */ },
  "images": { /* base64 images */ },
  "chapters": [
    {
      "type": "View",
      "children": [
        {
          "type": "Text",
          "content": "Chapter content",
          "styles": { "fontSize": 16 }
        }
      ]
    }
  ]
}
```

---

## Development Setup

```bash
# Install dependencies
pnpm install

# Build WASM module
pnpm build:wasm

# Build TypeScript package
pnpm build:ts

# Run example server
cd examples/v80-server
pnpm dev
```

---

## Contributing

This project is currently **not maintained**, but contributions are welcome if you want to develop it further:

1. Fork the repository
2. Extend the WASM parsing capabilities
3. Improve React Native integration
4. Prepare for Node API support

## License

MIT - Feel free to use, modify, and distribute as needed.