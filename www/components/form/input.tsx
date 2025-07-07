import clsx from "clsx";
import Form from ".";

type InputProps = {
  label: string;
  type: "text" | "password";
  id?: string;
  name?: string;
  placeholder?: string;
  error?: string;
  onInput: (value: string) => void;
  required?: boolean;
};

export function Input({
  label,
  name,
  type = "text",
  placeholder,
  error,
  onInput,
  required,
}: InputProps) {
  return (
    <label className="flex flex-col gap-2 mb-3">
      <span className="text-base">{label}</span>
      {error && <Form.AlertaDeErro>{error}</Form.AlertaDeErro>}
      <input
        type={type}
        name={name}
        placeholder={placeholder}
        className={clsx(
          "transition-all duration-150 rounded-3xl px-4 py-2 border border-black/20",
          "leading-none placeholder:text-black/60",
          "outline-none ring-yellow-500 ring-0 focus:ring-4",
          "focus:border-yellow-500",
        )}
        onInput={(event) => onInput(event.currentTarget.value)}
        required={required}
      />
    </label>
  );
}
