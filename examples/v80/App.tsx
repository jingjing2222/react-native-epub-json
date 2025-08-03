/**
 * @format
 */

import type React from 'react';
import { useEffect, useState } from 'react';
import {
  ActivityIndicator,
  Image,
  SafeAreaView,
  ScrollView,
  StatusBar,
  StyleSheet,
  Text,
  useColorScheme,
  View,
} from 'react-native';

// JSON_TYPES.md 기반 타입 정의
interface RnStyles {
  // 폰트 관련
  fontSize?: number;
  fontWeight?:
    | 'normal'
    | 'bold'
    | '100'
    | '200'
    | '300'
    | '400'
    | '500'
    | '600'
    | '700'
    | '800'
    | '900';
  fontFamily?: string;
  fontStyle?: 'normal' | 'italic';

  // 색상
  color?: string;
  backgroundColor?: string;

  // 텍스트 정렬
  textAlign?: 'auto' | 'left' | 'right' | 'center' | 'justify';
  lineHeight?: number;
  textDecorationLine?:
    | 'none'
    | 'underline'
    | 'line-through'
    | 'underline line-through';

  // 여백 (Margin)
  marginTop?: number;
  marginBottom?: number;
  marginLeft?: number;
  marginRight?: number;

  // 안쪽 여백 (Padding)
  paddingTop?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  paddingRight?: number;
}

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
  source: string; // base64 data URI
  alt?: string;
  styles?: RnStyles;
}

type RnNode = TextNode | ViewNode | ImageNode;

// test change

interface ChapterStructure {
  spine_index: number;
  idref: string;
  title?: string;
  content: RnNode;
}

interface EpubMetadata {
  title?: string;
  author?: string;
}

interface CompleteEpubInfo {
  metadata: EpubMetadata;
  chapters: ChapterStructure[];
  styles: Record<string, RnStyles>;
  images: Record<string, string>;
}

// RnNode를 React Native 컴포넌트로 변환하는 재귀 함수
const renderNode = (node: RnNode, index: number): React.ReactNode => {
  const key = `${node.type}-${index}`;
  switch (node.type) {
    case 'Text':
      return (
        <Text key={key} style={node.styles}>
          {node.content}
        </Text>
      );

    case 'View':
      return (
        <View key={key} style={node.styles}>
          {node.children.map((child, i) => renderNode(child, i))}
        </View>
      );

    case 'Image': {
      return (
        <Image key={key} source={{ uri: node.source }} resizeMode="contain" />
      );
    }

    default:
      return null;
  }
};

function App() {
  const isDarkMode = useColorScheme() === 'dark';
  const [book, setBook] = useState<CompleteEpubInfo | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchBook = async () => {
      try {
        const response = await fetch('http://localhost:8080/');
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: CompleteEpubInfo = await response.json();
        setBook(data);
      } catch (e: unknown) {
        if (e instanceof Error) {
          setError(`Failed to fetch book: ${e.message}`);
        } else {
          setError(`Failed to fetch book: ${String(e)}`);
        }
        console.error(e);
      } finally {
        setLoading(false);
      }
    };

    fetchBook();
  }, []);

  const backgroundStyle = {
    backgroundColor: isDarkMode ? '#333' : '#FFF',
    flex: 1,
  };

  const textColor = {
    color: isDarkMode ? '#FFF' : '#000',
  };

  if (loading) {
    return (
      <View style={[styles.container, styles.center]}>
        <ActivityIndicator size="large" />
        <Text style={textColor}>Loading Book...</Text>
      </View>
    );
  }

  if (error) {
    return (
      <View style={[styles.container, styles.center]}>
        <Text style={styles.errorText}>Error</Text>
        <Text style={textColor}>{error}</Text>
      </View>
    );
  }

  if (!book) {
    return (
      <View style={[styles.container, styles.center]}>
        <Text style={textColor}>No book data found.</Text>
      </View>
    );
  }

  return (
    <SafeAreaView style={backgroundStyle}>
      <StatusBar barStyle={isDarkMode ? 'light-content' : 'dark-content'} />
      <ScrollView
        contentInsetAdjustmentBehavior="automatic"
        style={backgroundStyle}
      >
        <View style={styles.header}>
          <Text style={[styles.title, textColor]}>{book.metadata.title}</Text>
          <Text style={[styles.author, textColor]}>{book.metadata.author}</Text>
        </View>
        {book.chapters.slice(0, 4).map((chapter, index) => (
          <View key={chapter.idref || index} style={styles.chapterContainer}>
            {renderNode(chapter.content, 0)}
          </View>
        ))}
      </ScrollView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  center: {
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  header: {
    padding: 20,
    borderBottomWidth: 1,
    borderBottomColor: '#CCC',
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    textAlign: 'center',
  },
  author: {
    fontSize: 18,
    textAlign: 'center',
    marginTop: 4,
  },
  chapterContainer: {
    padding: 16,
    borderBottomWidth: 1,
    borderBottomColor: '#EAEAEA',
  },
  errorText: {
    color: 'red',
    fontSize: 18,
    marginBottom: 10,
  },
});

export default App;
// test change
// test change
