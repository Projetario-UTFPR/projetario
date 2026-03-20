import "@/app.css";
import { createInertiaApp } from "@inertiajs/react";
import { NuqsAdapter } from "nuqs/adapters/react";
import { createRoot, hydrateRoot } from "react-dom/client";
import { resolvePage, resolveTitle } from "@/lib/inertia";

createInertiaApp({
  progress: { includeCSS: true, color: "#FFB300" },

  title: resolveTitle,

  resolve: resolvePage,

  setup: ({ App, el, props }) => {
    const isSSR = document.head.querySelector("meta[name='ssr']")?.getAttribute("content") === "true";

    const element = (
      <NuqsAdapter>
        <App {...props} />
      </NuqsAdapter>
    );

    isSSR ? hydrateRoot(el, element) : createRoot(el).render(element);
  },
});
