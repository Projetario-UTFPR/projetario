import type { TemaEstrito } from "@/tema";
import { TinyMCEEditor } from "../tinymce-editor";
import { AlertaDeErro } from "./alerta-de-erro";
import { InputLabelSpan } from "./label-span";

type Props = {
  tema?: TemaEstrito;
  label: string;
  required?: boolean;
  initialValue?: string;
  value?: string;
  error?: string;
  atualizarConteudo: (conteudo: string) => void;
};

export function Editor({
  tema = "claro",
  label,
  required,
  error,
  value,
  atualizarConteudo,
  initialValue,
}: Props) {
  return (
    <div className="flex flex-col gap-2">
      <InputLabelSpan required={required}>{label}</InputLabelSpan>
      {error && <AlertaDeErro>{error}</AlertaDeErro>}
      <TinyMCEEditor
        tema={tema}
        initialValue={initialValue}
        value={value}
        onEditorChange={(html, _editor) => atualizarConteudo(html)}
      />
    </div>
  );
}
