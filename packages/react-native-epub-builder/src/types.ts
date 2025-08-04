import { CompleteEpubInfo, RnNode } from "react-native-epub-json";
import { ViewStyle } from "react-native";

export interface EPUBReaderProps {
  data: CompleteEpubInfo;
  style?: ViewStyle;
  onChapterChange?: (chapterIndex: number) => void;
  renderCustomComponent?: (node: RnNode) => React.ReactElement | null;
}

export interface EPUBPageProps {
  data: CompleteEpubInfo;
  initialChapter?: number;
  showTOC?: boolean;
  style?: ViewStyle;
  onTOCPress?: (chapterIndex: number) => void;
}
