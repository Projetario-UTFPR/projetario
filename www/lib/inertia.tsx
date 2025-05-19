import type { PageResolver } from "@inertiajs/core";
import type { ReactElement } from "react";
import type { JSX } from "react";

import { DefaultLayout } from "@/layouts/default";

type PageComponent = ReactElement & {
  default: { layout: (_page: JSX.Element) => ReactElement };
};

const appName = "Inertia Rust";

export const resolveTitle = (title?: string) => title
  ? `${appName} - ${title}`
  : appName;

export const resolvePage: PageResolver = async (name: string) => {
  const pages = import.meta.glob("../pages/**/*.tsx", { eager: false });
  const pagePromise = pages[`../pages/${name}.tsx`];

  if (!pagePromise) throw new Error(`Could not find page ${name}.`);

  const page = await pagePromise() as PageComponent;

  page.default.layout ??= (page: JSX.Element) => (<DefaultLayout>{page}</DefaultLayout>);

  return page;
};

declare module "@inertiajs/core" {
  export interface PageProps {
    flash: Record<string, string>;
  }
}
