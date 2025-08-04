import React, { useState, useMemo } from "react";
import { View, ScrollView, Text, StyleSheet } from "react-native";
import { ComponentRenderer } from "./ComponentRenderer";
import { EPUBReaderProps } from "../types";

export const EPUBReader: React.FC<EPUBReaderProps> = ({
  data,
  style,
  onChapterChange,
  renderCustomComponent,
}) => {
  const [currentChapterIndex, setCurrentChapterIndex] = useState(0);

  const currentChapter = useMemo(() => {
    return data.chapters[currentChapterIndex];
  }, [data.chapters, currentChapterIndex]);

  const handleChapterChange = (newIndex: number) => {
    if (newIndex >= 0 && newIndex < data.chapters.length) {
      setCurrentChapterIndex(newIndex);
      onChapterChange?.(newIndex);
    }
  };

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
    textAlign: "center",
    fontSize: 16,
    color: "#666",
    marginTop: 50,
  },
});
