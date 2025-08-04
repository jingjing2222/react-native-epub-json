import type React from 'react';
import { useMemo, useState } from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import type { EPUBReaderProps } from '../types';
import { ComponentRenderer } from './ComponentRenderer';

export const EPUBReader: React.FC<EPUBReaderProps> = ({
  data,
  style,
  renderCustomComponent,
}) => {
  const [currentChapterIndex, _setCurrentChapterIndex] = useState(0);

  const currentChapter = useMemo(() => {
    return data.chapters[currentChapterIndex];
  }, [data.chapters, currentChapterIndex]);

  if (!currentChapter) {
    return (
      <View style={[styles.container, style]}>
        <Text style={styles.errorText}>No chapter content available</Text>
      </View>
    );
  }

  return (
    <View style={[styles.container, style]}>
      <ScrollView
        style={styles.scrollView}
        contentContainerStyle={styles.contentContainer}
      >
        <ComponentRenderer
          node={currentChapter.content}
          renderCustomComponent={renderCustomComponent}
        />
      </ScrollView>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollView: {
    flex: 1,
  },
  contentContainer: {
    padding: 16,
  },
  errorText: {
    textAlign: 'center',
    fontSize: 16,
    color: '#666',
    marginTop: 50,
  },
});
