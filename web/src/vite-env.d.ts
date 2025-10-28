// File: web/src/vite-env.d.ts
// TypeScript'e SVG dosyalarını bir modül olarak tanımasını söyler.

/// <reference types="vite/client" />

declare module '*.svg' {
  const content: any;
  export default content;
}