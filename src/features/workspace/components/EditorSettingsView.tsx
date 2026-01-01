import type { Accessor } from "solid-js";
import { Select } from "../../../ui/components/Select";

export type EditorOption = {
  value: string;
  label: string;
};

export type EditorSettingsViewProps = {
  contentOptions: Accessor<EditorOption[]>;
  inputOptions: Accessor<EditorOption[]>;
};

export const EditorSettingsView = (props: EditorSettingsViewProps) => {
  return (
    <div>
      <Select
        value={props.contentOptions()[0]?.value ?? ""}
        options={props.contentOptions()}
        onChange={() => {}}
      />
      <Select
        value={props.inputOptions()[0]?.value ?? ""}
        options={props.inputOptions()}
        onChange={() => {}}
      />
    </div>
  );
};
