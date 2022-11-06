import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from "path"
import styleImport, { AntdResolve } from "vite-plugin-style-import"


export default defineConfig({
  plugins: [
    react(),
    styleImport(
      {
        resolves: [
          AntdResolve(),
        ]
      }
    )
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src")
    }
  },
  css: {
    preprocessorOptions: {
      less: {
        javascriptEnabled: true,
      },
    }
  },
  server: {
    host: "0.0.0.0", // 服务器主机名，如果允许外部访问，可设置为"0.0.0.0"
    port: 3715,
    open: true,
    cors: true,
    // https: false,
    // proxy: {
    //   "/dev-api": {
    //     target: "http://127.0.0.1:8080", // easymock
    //     changeOrigin: true,
    //     rewrite: path => path.replace(/^\/dev-api/, "")
    //   },
    //   '/prod-api': {
    //     target: 'http://127.0.0.1:8080',
    //     changeOrigin: true,
    //     rewrite: path => path.replace(/^\/prod-api/, '')
    //   }
    // }
  },
})
