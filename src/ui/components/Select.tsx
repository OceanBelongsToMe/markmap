export type SelectOption = {
  value: string;
  label: string;
};

export type SelectProps = {
  id?: string;
  value: string;
  options: SelectOption[];
  onChange: (value: string) => void;
};

export const Select = (props: SelectProps) => {
  return (
    <select
      id={props.id}
      value={props.value}
      onChange={(e) => props.onChange(e.currentTarget.value)}
    >
      {props.options.map((option) => (
        <option value={option.value}>{option.label}</option>
      ))}
    </select>
  );
};
