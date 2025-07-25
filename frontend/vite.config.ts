import { defineConfig } from "vite";

import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";
// import path from "path";
import path from "path";

// https://vite.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  plugins: [vue(), tailwindcss()],
  build: {
    assetsDir: "assets/blog",
    rollupOptions: {
      output: {
        assetFileNames: "assets/blog/[name]-[hash][extname]",
        chunkFileNames: "assets/blog/[name]-[hash].js",
      },
    },
  },
});
