import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import tsconfigPaths from "vite-tsconfig-paths";
import { fileURLToPath } from "node:url";

// https://vitejs.dev/config/
export default defineConfig({
  base: "/dashboard",
  plugins: [react(), tsconfigPaths()],
  build: {
    rollupOptions: {
      input: {
        mookbark: fileURLToPath(new URL("./index.html", import.meta.url)),
      },
      output: { dir: "./dist" },
    },
  },
});
