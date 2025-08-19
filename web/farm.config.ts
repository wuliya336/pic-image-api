import { defineConfig } from "@farmfe/core";
import { fileURLToPath } from "node:url";
import path from "node:path";
import postcss from "@farmfe/js-plugin-postcss";

const filePath = path.dirname(fileURLToPath(import.meta.url));

/**
 * Farm 构建工具的配置文件
 * @see https://farmfe.github.io/docs/configure
 */
export default defineConfig({
  server: {
    port: 33710,
    hmr: true,
  },
  // 编译配置
  compilation: {
    // 脚本相关配置
    script: {
      // 解析器配置
      parser: {
        // TypeScript 解析配置
        tsConfig: {
          decorators: true, // 启用 TypeScript 装饰器语法支持
        },
      },
    },
    output: {
      path: path.join(filePath, "dist"),
    },
    sourcemap: false, // 源码映射
    resolve: {
      alias: {
        "@": path.resolve(filePath, "src"),
      },
    },
  },
  // 插件配置
  plugins: [
    "@farmfe/plugin-react", // React 插件，用于支持 React 项目
    ["@farmfe/plugin-sass", { sourceMap: false }], // Sass 插件，用于支持 Sass/SCSS 预处理
    postcss(),
  ],
});