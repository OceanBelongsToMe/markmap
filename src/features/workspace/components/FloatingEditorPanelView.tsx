import type { Accessor } from "solid-js";
import { Button } from "../../../ui/components/Button";
import { TextInput } from "../../../ui/components/TextInput";

export type FloatingEditorPanelViewProps = {
  title: string;
  placeholder: string;
  closeLabel: string;
  isOpen: Accessor<boolean>;
  onClose: () => void;
  onRef: (el: HTMLDivElement) => void;
};

export const FloatingEditorPanelView = (props: FloatingEditorPanelViewProps) => {
  if (!props.isOpen()) return null;

  return (
    <div ref={props.onRef}>
      <div>{props.title}</div>
      <TextInput placeholder={props.placeholder} autoFocus />
      <Button onClick={props.onClose}>{props.closeLabel}</Button>
    </div>
  );
};
