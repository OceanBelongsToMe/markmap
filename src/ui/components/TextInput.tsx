export type TextInputProps = {
  id?: string;
  value?: string;
  placeholder?: string;
  autoFocus?: boolean;
  onChange?: (value: string) => void;
};

export const TextInput = (props: TextInputProps) => {
  let inputRef: HTMLInputElement | undefined;

  if (props.autoFocus) {
    queueMicrotask(() => inputRef?.focus());
  }

  const valueProps = props.value !== undefined ? { value: props.value } : {};

  return (
    <input
      id={props.id}
      ref={inputRef}
      {...valueProps}
      placeholder={props.placeholder}
      onChange={(e) => props.onChange?.(e.currentTarget.value)}
    />
  );
};
