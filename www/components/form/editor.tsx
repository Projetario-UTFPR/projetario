import { memo, Suspense, use } from "react";
import type { TemaEstrito } from "@/tema";
import type { TinyMCEEditorProps } from "../tinymce-editor";
import { InputLabelSpan } from "./label-span";

// colocar essa promise inline (`use(import("path"))`) faz com que uma nova promise seja
// gerada a cada renderização, pode ocasionar um loop de re-renderização deste componente.
const promiseEstavel = import("../tinymce-editor");

const LazyTinyMceEditor = memo((props: TinyMCEEditorProps) => {
  const { TinyMCEEditor } = use(promiseEstavel);
  return <TinyMCEEditor {...props} />;
});

const EditorSkeleton = () => {
  return (
    <div className="text-input inert select-none animate-pulse opacity-50 px-3">
      <span className="block rounded-full bg-black/10 dark:bg-white/10 w-1/4">&nbsp;</span>
    </div>
  );
};

type Props = {
  tema?: TemaEstrito;
  label: string;
  required?: boolean;
  initialValue?: string;
  erro?: string;
  atualizarConteudo?: (conteudo: string) => void;
};

export const Editor = memo(
  ({ tema = "claro", label, required, erro, atualizarConteudo, initialValue }: Props) => {
    return (
      <div className="flex flex-col gap-2">
        <InputLabelSpan required={required}>{label}</InputLabelSpan>
        <Suspense fallback={<EditorSkeleton />}>
          <LazyTinyMceEditor
            tema={tema}
            erro={erro}
            initialValue={initialValue}
            onEditorChange={(html, _editor) => atualizarConteudo?.(html)}
          />
        </Suspense>
      </div>
    );
  },
);
