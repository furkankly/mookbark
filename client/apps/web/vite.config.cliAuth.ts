import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import tsconfigPaths from "vite-tsconfig-paths";
import { fileURLToPath } from "node:url";

// https://vitejs.dev/config/
export default defineConfig({
  base: "/",
  plugins: [react(), tsconfigPaths()],
  build: {
    rollupOptions: {
      input: {
        mookbarkCliAuth: fileURLToPath(
          new URL("./cliAuth/index.html", import.meta.url)
        ),
      },
      output: { dir: "./dist-cliAuth" },
    },
  },
  server: { port: 9999 },
});
