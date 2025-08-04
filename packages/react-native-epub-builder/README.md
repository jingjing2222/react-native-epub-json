# @react-native-epub-builder/core

React Native EPUB viewer components built on top of `react-native-epub-json`.

## Installation

```bash
npm install @react-native-epub-builder/core react-native-epub-json
# or
yarn add @react-native-epub-builder/core react-native-epub-json
```

## Usage

### EPUBPage (Complete Page Component)

```tsx
import React from 'react';
import { EPUBPage } from '@react-native-epub-builder/core';
import { epubToJson } from 'react-native-epub-json';

const App = () => {
  const epubData = epubToJson('/path/to/book.epub', '/output/dir');
  
  return (
    <EPUBPage 
      data={epubData}
      showTOC={true}
      onTOCPress={(chapterIndex) => console.log('Chapter selected:', chapterIndex)}
    />
  );
};
```

### EPUBReader (Reader Component Only)

```tsx
import React from 'react';
import { EPUBReader } from '@react-native-epub-builder/core';

const App = () => {
  return (
    <EPUBReader
      data={epubData}
      onChapterChange={(index) => console.log('Chapter changed:', index)}
      renderCustomComponent={(node) => {
        // Custom rendering logic
        return null; // Return null to use default rendering
      }}
    />
  );
};
```

## Components

### EPUBPage
Complete EPUB reading experience with navigation and table of contents.

**Props:**
- `data: CompleteEpubInfo` - EPUB data from react-native-epub-json
- `initialChapter?: number` - Initial chapter index (default: 0)
- `showTOC?: boolean` - Show table of contents button (default: true)
- `style?: ViewStyle` - Custom container styles
- `onTOCPress?: (chapterIndex: number) => void` - Chapter selection callback

### EPUBReader
Core reading component for custom implementations.

**Props:**
- `data: CompleteEpubInfo` - EPUB data from react-native-epub-json
- `style?: ViewStyle` - Custom container styles
- `onChapterChange?: (chapterIndex: number) => void` - Chapter change callback
- `renderCustomComponent?: (node: RnNode) => React.ReactElement | null` - Custom component renderer

### ComponentRenderer
Low-level component for rendering individual nodes.

### TOCModal
Table of contents modal component.

## License

MIT