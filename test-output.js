const path = require("path");
const { epubToJson } = require("./packages/react-native-epub-json/dist/index.js");

const epubFilePath = path.join(__dirname, "packages/react-native-epub-json-rust/hemingway.epub");
const outputDir = path.join(__dirname, "packages/react-native-epub-json/test_output");

try {
  console.log(`🚀 Testing epubToJson...`);
  console.log(`📖 Input EPUB: ${epubFilePath}`);
  console.log(`📂 Output directory: ${outputDir}`);

  const result = epubToJson(epubFilePath, outputDir);

  if (result && result.metadata && result.metadata.title) {
    console.log(`✅ Test successful!`);
    console.log(`   - Book Title: ${result.metadata.title}`);
    console.log(`   - JSON output saved in: ${outputDir}`);
  } else {
    console.error("❌ Test failed: The returned JSON is invalid or empty.");
    console.log("Returned object:", JSON.stringify(result, null, 2));
  }
} catch (error) {
  console.error("❌ An error occurred during the test:");
  console.error(error);
  process.exit(1);
}
