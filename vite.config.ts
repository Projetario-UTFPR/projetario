import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import laravel from "laravel-vite-plugin";
import type { UserConfig } from "vite";

export default {
  plugins: [
    tailwindcss(),
    react(),
    laravel({
      input: ["www/app.tsx"],
      buildDirectory: "bundle",
      ssrOutputDirectory: "dist/ssr",
      ssr: "www/ssr.tsx",
    }),
  ],
  publicDir: "/public",
  server: {
    watch: {
      ignored: ["*"],
    },
  },
} satisfies UserConfig;
