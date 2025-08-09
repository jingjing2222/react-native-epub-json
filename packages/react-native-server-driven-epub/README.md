# React Native Server-Driven EPUB

Convert EPUB files to React Native server-driven UI components with WASM-powered parsing.

## Features

- 🚀 **WASM-powered**: Fast EPUB parsing using Rust and WebAssembly
- 📱 **React Native Ready**: Converts EPUB content to React Native component structures
- 🎨 **Style Conversion**: CSS styles converted to React Native StyleSheet format
- 🖼️ **Image Support**: Base64 embedded images with proper React Native Image component support
- 📖 **Table of Contents**: Structured TOC with navigation support
- 🌐 **Server-Driven**: Perfect for server-driven UI architectures

## Installation

```bash
npm install react-native-server-driven-epub
# or
yarn add react-native-server-driven-epub
# or
pnpm add react-native-server-driven-epub
```

## Usage

### Basic Usage

```typescript
import { epubToJson, epubBytesToJson } from 'react-native-server-driven-epub';

// Convert from file path
const epubData = epubToJson('./book.epub', './output');

// Convert from bytes (useful for server environments)
const fileBuffer = fs.readFileSync('./book.epub');
const epubData = epubBytesToJson(new Uint8Array(fileBuffer));
```

### Server Example

```typescript
import { epubBytesToJson } from 'react-native-server-driven-epub';
import { Hono } from 'hono';

const app = new Hono();

app.get('/epub/:id', async (c) => {
  const epubPath = `./books/${c.req.param('id')}.epub`;
  const fileBuffer = fs.readFileSync(epubPath);
  const jsonData = epubBytesToJson(new Uint8Array(fileBuffer));
  
  return c.json(jsonData);
});
```

## Output Structure

The library converts EPUB files into a structured JSON format:

```typescript
interface CompleteEpubInfo {
  metadata: EpubMetadata;      // Book metadata (title, author, etc.)
  structure: EpubStructure;    // Book structure info
  toc: TocItem[];             // Table of contents
  spine: SpineItemInfo[];     // Reading order
  styles: Record<string, RnStyles>; // React Native styles
  images: Record<string, string>;   // Base64 encoded images
  chapters: ChapterStructure[];     // Chapter content as RN components
}
```

### React Native Component Structure

Each chapter is converted to React Native component nodes:

```typescript
type RnNode = TextNode | ViewNode | ImageNode | ScrollViewNode;

interface TextNode {
  type: 'Text';
  content: string;
  styles?: RnStyles;
}

interface ViewNode {
  type: 'View';
  children: RnNode[];
  styles?: RnStyles;
}

interface ImageNode {
  type: 'Image';
  source: string; // Base64 data URI
  alt?: string;
  styles?: RnStyles;
}
```

## React Native Rendering

You can render the converted structure in your React Native app:

```tsx
import React from 'react';
import { View, Text, Image, ScrollView } from 'react-native';

const renderNode = (node: RnNode): React.ReactElement => {
  switch (node.type) {
    case 'Text':
      return <Text style={node.styles}>{node.content}</Text>;
    
    case 'View':
      return (
        <View style={node.styles}>
          {node.children.map((child, index) => (
            <React.Fragment key={index}>{renderNode(child)}</React.Fragment>
          ))}
        </View>
      );
    
    case 'Image':
      return <Image source={{ uri: node.source }} style={node.styles} />;
    
    case 'ScrollView':
      return (
        <ScrollView style={node.styles}>
          {node.children.map((child, index) => (
            <React.Fragment key={index}>{renderNode(child)}</React.Fragment>
          ))}
        </ScrollView>
      );
  }
};

const EpubReader = ({ epubData }: { epubData: CompleteEpubInfo }) => {
  return (
    <ScrollView>
      {epubData.chapters.map((chapter, index) => (
        <View key={index}>
          {renderNode(chapter.content)}
        </View>
      ))}
    </ScrollView>
  );
};
```

## API Reference

### `epubToJson(epubPath: string, outputDir: string): CompleteEpubInfo`

Converts an EPUB file to JSON and saves it to a file.

- `epubPath`: Path to the EPUB file
- `outputDir`: Directory to save the output JSON file
- Returns: The converted JSON object

### `epubToJsonString(epubPath: string): string`

Converts an EPUB file to a JSON string.

- `epubPath`: Path to the EPUB file
- Returns: JSON string representation

### `epubBytesToJson(epubBytes: Uint8Array): CompleteEpubInfo`

Converts EPUB bytes from memory into a JSON object.

- `epubBytes`: The EPUB file content as a byte array
- Returns: The converted JSON object

## Requirements

- Node.js 16 or higher
- React Native 0.60 or higher (for proper React Native component support)

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.