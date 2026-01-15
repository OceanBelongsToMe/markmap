import Code from "lucide-solid/icons/code";
import ChartNetwork from "lucide-solid/icons/chart-network";
import { ToggleGroup } from "@ark-ui/solid/toggle-group";
import "./editor-view-toggle.css";

export type EditorViewMode = "code" | "markmap";

export type EditorViewModeToggleProps = {
  value: EditorViewMode;
  onChange: (value: EditorViewMode) => void;
};

export const EditorViewModeToggle = (props: EditorViewModeToggleProps) => {
  return (
    <ToggleGroup.Root
      class="editor-view-toggle"
      type="single"
      value={[props.value]}
      aria-label="Editor view mode"
      onValueChange={(details) => {
        const next = details.value?.[0];
        if (next) {
          props.onChange(next as EditorViewMode);
        }
      }}
    >
      <ToggleGroup.Item
        class="editor-view-toggle-item"
        value="code"
        title="Code"
        aria-label="Code"
      >
        <Code size={14} />
      </ToggleGroup.Item>
      <ToggleGroup.Item
        class="editor-view-toggle-item"
        value="markmap"
        title="Mind Map"
        aria-label="Mind Map"
      >
        <ChartNetwork size={14} />
      </ToggleGroup.Item>
    </ToggleGroup.Root>
  );
};
