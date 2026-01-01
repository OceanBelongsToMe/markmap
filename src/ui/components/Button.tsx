import type { JSX } from "solid-js";

export type ButtonProps = {
  type?: "button" | "submit" | "reset";
  onClick?: () => void;
  children: JSX.Element;
};

export const Button = (props: ButtonProps) => {
  return (
    <button
      type={props.type ?? "button"}
      onClick={props.onClick}
    >
      {props.children}
    </button>
  );
};
