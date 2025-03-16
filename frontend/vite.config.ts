import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'
import path from "path"

export default defineConfig({
  plugins: [solid()],
  resolve: {
    alias: {
      '~bootstrap': path.resolve(__dirname, 'node_modules/bootstrap'),
    }
  },
  server: {
    port: 8080,
    proxy: {
      '/api': 'http://localhost:8000'
    }
  }
})
