import Form from ".";
import { InputLabelSpan } from "./label-span";

type Value = React.InputHTMLAttributes<HTMLInputElement>["value"];

type InputProps = {
  label: string;
  type: "text" | "password" | "date" | "datetime-local" | "number";
  id?: string;
  name?: string;
  placeholder?: string;
  error?: string;
  value?: Value;
  onInput?: (value: string) => void;
  /** Adiciona um pequeno texto de observação em relação a esse campo de texto. */
  observacao?: string;
  required?: boolean;
};

export function Input({
  id,
  label,
  name,
  type = "text",
  placeholder,
  error,
  value,
  onInput,
  required,
  observacao,
}: InputProps) {
  return (
    <label className="flex flex-col gap-2 mb-3">
      <InputLabelSpan required={required}>{label}</InputLabelSpan>

      {error && <Form.AlertaDeErro>{error}</Form.AlertaDeErro>}

      {observacao && (
        <span className="text-sm text-gray-600 dark:text-gray-500">
          {observacao}
        </span>
      )}

      <input
        id={id}
        type={type}
        name={name}
        value={value}
        placeholder={placeholder}
        className="text-input leading-none"
        onInput={(event) => onInput?.(event.currentTarget.value)}
        required={required}
      />
    </label>
  );
}
