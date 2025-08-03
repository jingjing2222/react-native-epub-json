import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { serve } from '@hono/node-server';
import { Hono } from 'hono';

// ES 모듈 환경에서 CommonJS 모듈을 불러오기 위한 설정
const require = createRequire(import.meta.url);
const { epubBytesToJson } = require('react-native-epub-json');

const app = new Hono();

// 루트 경로에서 EPUB 파일을 실시간으로 JSON으로 변환하여 내려주기
app.get('/', (c) => {
  const epubPath =
    '/Users/kimhyeongjeong/Desktop/code/ebook/hemingway-old-man-and-the-sea.epub';

  // const epubPath = "/Users/kimhyeongjeong/Downloads/khj.epub";

  try {
    // 1. EPUB 파일 존재 확인
    if (!fs.existsSync(epubPath)) {
      console.error(`❌ EPUB 파일을 찾을 수 없습니다: ${epubPath}`);
      return c.text('EPUB 파일을 찾을 수 없습니다.', 404);
    }

    // 2. EPUB 파일을 버퍼로 읽기
    const fileBuffer = fs.readFileSync(epubPath);

    // 3. 버퍼를 Uint8Array로 변환하여 JSON 객체로 변환
    console.log(`📚 EPUB 변환 시작: ${path.basename(epubPath)}`);
    const jsonObject = epubBytesToJson(new Uint8Array(fileBuffer));
    console.log(`✅ 변환 성공 - 제목: "${jsonObject.metadata.title}"`);

    // 4. JSON 응답 전송
    return c.json(jsonObject);
  } catch (error) {
    console.error('❌ 파일 처리 또는 변환 오류:', error);
    return c.text('파일 처리 중 오류가 발생했습니다.', 500);
  }
});

// 서버 상태 확인
app.get('/health', (c) => {
  return c.json({
    status: 'ok',
    message: 'EPUB 서버가 정상 동작 중입니다.',
  });
});

const port = 8080;
console.log(`🚀 EPUB 서버가 http://localhost:${port} 에서 시작되었습니다.`);
console.log(`📖 EPUB JSON 요청: http://localhost:${port}/`);
console.log(`🔍 상태 확인: http://localhost:${port}/health`);

serve({
  fetch: app.fetch,
  port,
});
