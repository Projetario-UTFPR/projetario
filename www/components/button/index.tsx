import type { ButtonHTMLAttributes } from "react";
import { CallToAction } from "./cta";
import { Secundario } from "./secundario";

export type ButtonProps<T = ButtonHTMLAttributes<HTMLButtonElement>> = {
  asChild?: boolean;
} & T;

export default {
  CallToAction,
  Secundario,
};
