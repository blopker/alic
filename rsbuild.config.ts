import path from "node:path";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "@rsbuild/core";
import {pluginSolid} from "@rsbuild/plugin-solid";
import {pluginTailwindCSS} from "rsbuild-plugin-tailwindcss";
import { pluginBabel } from '@rsbuild/plugin-babel';
// const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    pluginBabel({
      include: /\.(?:jsx|tsx)$/,
    }),
    pluginSolid(), pluginTailwindCSS()],
  // build: {
  //   target: "safari15",
  // },

  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  // clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    // host: host || false,
    // hmr: host
    //   ? {
    //       protocol: "ws",
    //       host,
    //       port: 1421,
    //     }
    //   : undefined,
  },
  tools: {
        rspack: {
            watchOptions: {
                ignored: "**/src-tauri/**"
            }
        }
    }
});
