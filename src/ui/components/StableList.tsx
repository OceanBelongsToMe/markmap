import type { Accessor, JSX } from "solid-js";
import { Index } from "solid-js";

export type StableListProps<T> = {
  each: Accessor<T[]>;
  children: (item: Accessor<T>, index: Accessor<number>) => JSX.Element;
};

export const StableList = <T,>(props: StableListProps<T>) => {
  return <Index each={props.each()}>{props.children}</Index>;
};
