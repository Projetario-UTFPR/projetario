import { createInertiaApp } from "@inertiajs/react";
import createServer from "@inertiajs/react/server";
import ReactDOMServer from "react-dom/server";

import { resolvePage, resolveTitle } from "@/lib/inertia";

createServer((page) =>
  createInertiaApp({
    page,
    title: resolveTitle,
    render: ReactDOMServer.renderToString,
    resolve: resolvePage,
    setup: ({ App, props }) => <App {...props} />,
  }),
);
