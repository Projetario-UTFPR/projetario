import "@/app.css";
import { createInertiaApp } from "@inertiajs/react";
import { createRoot, hydrateRoot } from "react-dom/client";
import { resolveTitle, resolvePage } from "@/lib/inertia";

createInertiaApp({
  progress: { includeCSS: true, color: "#FFB300" },

  title: resolveTitle,

  resolve: resolvePage,

  setup: ({ App, el, props }) => {
    const isSSR =
      document.head
        .querySelector("meta[name='ssr']")
        ?.getAttribute("content") === "true";

    isSSR
      ? hydrateRoot(el, <App {...props} />)
      : createRoot(el).render(<App {...props} />);
  },
});
