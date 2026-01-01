export type LabelProps = {
  for?: string;
  text: string;
};

export const Label = (props: LabelProps) => {
  return <label for={props.for}>{props.text}</label>;
};
