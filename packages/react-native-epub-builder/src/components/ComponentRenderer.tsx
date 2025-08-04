import type React from 'react';
import type { ImageStyle, TextStyle, ViewStyle } from 'react-native';
import { Image, ScrollView, Text, View } from 'react-native';
import type { RnNode, RnStyles } from 'react-native-epub-json';

interface ComponentRendererProps {
  node: RnNode;
  renderCustomComponent?: (node: RnNode) => React.ReactElement | null;
}

const convertToReactNativeStyle = (
  styles: RnStyles | undefined,
): ViewStyle & TextStyle & ImageStyle => {
  if (!styles) return {};

  const converted: Record<string, unknown> = {};

  Object.entries(styles).forEach(([key, value]) => {
    if (value !== undefined) {
      // Fix specific type mismatches
      if (key === 'fontStyle' && typeof value === 'string') {
        converted[key] = value === 'italic' ? 'italic' : 'normal';
      } else if (key === 'borderStyle' && typeof value === 'string') {
        converted[key] = ['solid', 'dotted', 'dashed'].includes(value)
          ? value
          : 'solid';
      } else if (key === 'overflow' && typeof value === 'string') {
        converted[key] = ['visible', 'hidden'].includes(value)
          ? value
          : 'visible';
      } else {
        converted[key] = value;
      }
    }
  });

  return converted;
};

export const ComponentRenderer: React.FC<ComponentRendererProps> = ({
  node,
  renderCustomComponent,
}) => {
  if (renderCustomComponent) {
    const customComponent = renderCustomComponent(node);
    if (customComponent) {
      return customComponent;
    }
  }

  const renderNode = (currentNode: RnNode, index: number) => {
    const { type } = currentNode;
    const convertedStyles = convertToReactNativeStyle(currentNode.styles);

    switch (type) {
      case 'Text':
        return (
          <Text key={index} style={convertedStyles}>
            {currentNode.content}
          </Text>
        );

      case 'View':
        return (
          <View key={index} style={convertedStyles}>
            {currentNode.children?.map(renderNode)}
          </View>
        );

      case 'ScrollView':
        return (
          <ScrollView key={index} style={convertedStyles}>
            {currentNode.children?.map(renderNode)}
          </ScrollView>
        );

      case 'Image':
        return (
          <Image
            key={index}
            source={{ uri: currentNode.source }}
            style={convertedStyles}
            resizeMode="contain"
          />
        );

      default:
        return <View key={index} style={convertedStyles} />;
    }
  };

  return renderNode(node, 0);
};
