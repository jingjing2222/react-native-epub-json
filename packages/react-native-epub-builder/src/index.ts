export { EPUBReader } from "./components/EPUBReader";
export { EPUBPage } from "./components/EPUBPage";
export { ComponentRenderer } from "./components/ComponentRenderer";
export { TOCModal } from "./components/TOCModal";

export type { EPUBReaderProps, EPUBPageProps } from "./types";

// Re-export types from react-native-epub-json for convenience
export type {
  CompleteEpubInfo,
  EpubMetadata,
  EpubStructure,
  TocItem,
  SpineItemInfo,
  RnStyles,
  RnNode,
  TextNode,
  ViewNode,
  ImageNode,
  ScrollViewNode,
  ChapterStructure,
} from "react-native-epub-json";
