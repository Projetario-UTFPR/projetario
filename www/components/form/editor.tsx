import { TinyMCEEditor } from "../tinymce-editor";
import { AlertaDeErro } from "./alerta-de-erro";
import { InputLabelSpan } from "./label-span";

type Props = {
  label: string;
  required?: boolean;
  initialValue?: string;
  error?: string;
  atualizarCoteudo: (conteudo: string) => void;
};

export function Editor({
  label,
  required,
  error,
  atualizarCoteudo,
  initialValue,
}: Props) {
  return (
    <div className="flex flex-col gap-2">
      <InputLabelSpan required={required}>{label}</InputLabelSpan>
      {error && <AlertaDeErro>{error}</AlertaDeErro>}
      <TinyMCEEditor
        initialValue={initialValue}
        onEditorChange={(html, _editor) => atualizarCoteudo(html)}
      />
    </div>
  );
}
