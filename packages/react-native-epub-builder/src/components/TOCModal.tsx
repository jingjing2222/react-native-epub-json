import type React from 'react';
import {
  FlatList,
  Modal,
  SafeAreaView,
  StyleSheet,
  Text,
  TouchableOpacity,
  View,
} from 'react-native';
import type { CompleteEpubInfo } from 'react-native-epub-json';

interface TOCModalProps {
  visible: boolean;
  data: CompleteEpubInfo;
  onClose: () => void;
  onItemPress: (chapterIndex: number) => void;
  currentChapterIndex: number;
}

export const TOCModal: React.FC<TOCModalProps> = ({
  visible,
  data,
  onClose,
  onItemPress,
  currentChapterIndex,
}) => {
  const renderTOCItem = ({
    item,
    index,
  }: {
    item: (typeof data.toc)[0];
    index: number;
  }) => {
    const isActive = index === currentChapterIndex;

    return (
      <TouchableOpacity
        style={[styles.tocItem, isActive && styles.activeTocItem]}
        onPress={() => {
          onItemPress(index);
          onClose();
        }}
      >
        <Text style={[styles.tocLabel, isActive && styles.activeTocLabel]}>
          {item.label}
        </Text>
      </TouchableOpacity>
    );
  };

  return (
    <Modal
      visible={visible}
      animationType="slide"
      presentationStyle="pageSheet"
      onRequestClose={onClose}
    >
      <SafeAreaView style={styles.container}>
        <View style={styles.header}>
          <Text style={styles.title}>Table of Contents</Text>
          <TouchableOpacity onPress={onClose} style={styles.closeButton}>
            <Text style={styles.closeButtonText}>Close</Text>
          </TouchableOpacity>
        </View>

        <FlatList
          data={data.toc}
          renderItem={renderTOCItem}
          keyExtractor={(_, index) => index.toString()}
          style={styles.list}
        />
      </SafeAreaView>
    </Modal>
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
    padding: 16,
    borderBottomWidth: 1,
    borderBottomColor: '#e0e0e0',
  },
  title: {
    fontSize: 18,
    fontWeight: 'bold',
  },
  closeButton: {
    paddingHorizontal: 16,
    paddingVertical: 8,
  },
  closeButtonText: {
    color: '#007AFF',
    fontSize: 16,
  },
  list: {
    flex: 1,
  },
  tocItem: {
    paddingHorizontal: 16,
    paddingVertical: 12,
    borderBottomWidth: 1,
    borderBottomColor: '#f0f0f0',
  },
  activeTocItem: {
    backgroundColor: '#007AFF10',
  },
  tocLabel: {
    fontSize: 16,
    color: '#333',
  },
  activeTocLabel: {
    color: '#007AFF',
    fontWeight: '500',
  },
});
