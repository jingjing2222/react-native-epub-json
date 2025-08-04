import React, { useState } from 'react';
import { View, TouchableOpacity, Text, StyleSheet, SafeAreaView } from 'react-native';
import { EPUBReader } from './EPUBReader';
import { TOCModal } from './TOCModal';
import { EPUBPageProps } from '../types';

export const EPUBPage: React.FC<EPUBPageProps> = ({
  data,
  initialChapter = 0,
  showTOC = true,
  style,
  onTOCPress,
}) => {
  const [currentChapterIndex, setCurrentChapterIndex] = useState(initialChapter);
  const [showTOCModal, setShowTOCModal] = useState(false);

  const handleChapterChange = (newIndex: number) => {
    setCurrentChapterIndex(newIndex);
    onTOCPress?.(newIndex);
  };

  const handleTOCPress = () => {
    setShowTOCModal(true);
  };

  const handleTOCItemPress = (chapterIndex: number) => {
    setCurrentChapterIndex(chapterIndex);
    onTOCPress?.(chapterIndex);
  };

  return (
    <SafeAreaView style={[styles.container, style]}>
      <View style={styles.header}>
        <Text style={styles.title} numberOfLines={1}>
          {data.metadata.title || 'EPUB Reader'}
        </Text>
        {showTOC && (
          <TouchableOpacity onPress={handleTOCPress} style={styles.tocButton}>
            <Text style={styles.tocButtonText}>Contents</Text>
          </TouchableOpacity>
        )}
      </View>

      <EPUBReader
        data={data}
        style={styles.reader}
        onChapterChange={handleChapterChange}
      />

      <View style={styles.navigation}>
        <TouchableOpacity
          style={[styles.navButton, currentChapterIndex === 0 && styles.disabledButton]}
          onPress={() => handleChapterChange(currentChapterIndex - 1)}
          disabled={currentChapterIndex === 0}
        >
          <Text style={[styles.navButtonText, currentChapterIndex === 0 && styles.disabledText]}>
            Previous
          </Text>
        </TouchableOpacity>

        <Text style={styles.pageInfo}>
          {currentChapterIndex + 1} / {data.chapters.length}
        </Text>

        <TouchableOpacity
          style={[
            styles.navButton,
            currentChapterIndex === data.chapters.length - 1 && styles.disabledButton,
          ]}
          onPress={() => handleChapterChange(currentChapterIndex + 1)}
          disabled={currentChapterIndex === data.chapters.length - 1}
        >
          <Text
            style={[
              styles.navButtonText,
              currentChapterIndex === data.chapters.length - 1 && styles.disabledText,
            ]}
          >
            Next
          </Text>
        </TouchableOpacity>
      </View>

      <TOCModal
        visible={showTOCModal}
        data={data}
        onClose={() => setShowTOCModal(false)}
        onItemPress={handleTOCItemPress}
        currentChapterIndex={currentChapterIndex}
      />
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: 16,
    paddingVertical: 12,
    borderBottomWidth: 1,
    borderBottomColor: '#e0e0e0',
  },
  title: {
    flex: 1,
    fontSize: 18,
    fontWeight: 'bold',
    marginRight: 16,
  },
  tocButton: {
    paddingHorizontal: 12,
    paddingVertical: 6,
    backgroundColor: '#007AFF',
    borderRadius: 6,
  },
  tocButtonText: {
    color: '#fff',
    fontSize: 14,
    fontWeight: '500',
  },
  reader: {
    flex: 1,
  },
  navigation: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: 16,
    paddingVertical: 12,
    borderTopWidth: 1,
    borderTopColor: '#e0e0e0',
  },
  navButton: {
    paddingHorizontal: 16,
    paddingVertical: 8,
    backgroundColor: '#007AFF',
    borderRadius: 6,
    minWidth: 80,
    alignItems: 'center',
  },
  disabledButton: {
    backgroundColor: '#ccc',
  },
  navButtonText: {
    color: '#fff',
    fontSize: 14,
    fontWeight: '500',
  },
  disabledText: {
    color: '#888',
  },
  pageInfo: {
    fontSize: 14,
    color: '#666',
  },
});