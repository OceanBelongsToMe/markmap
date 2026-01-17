import { createEffect, onCleanup, untrack } from "solid-js";
import { EditorState, Extension } from "@codemirror/state";
import { EditorView } from "@codemirror/view";
import { basicSetup } from "codemirror";

interface UseCodeMirrorProps {
  container: HTMLDivElement | undefined;
  value?: string;
  extensions?: Extension[];
  onChange?: (value: string) => void;
  autoFocus?: boolean;
  /** @default true */
  useBasicSetup?: boolean;
}

export function useCodeMirror(props: () => UseCodeMirrorProps) {
  let view: EditorView | undefined;

  const updateView = (value: string) => {
    if (!view) return;
    const currentValue = view.state.doc.toString();
    if (value !== currentValue) {
      view.dispatch({
        changes: { from: 0, to: currentValue.length, insert: value },
      });
    }
  };

  createEffect(() => {
    const { container } = props();

    if (!container) return;
    if (view) return; // Prevent re-initialization if already exists

    // Use untrack to prevent re-running this effect when value/extensions change
    const currentProps = untrack(props);
    const { useBasicSetup = true } = currentProps;

    const state = EditorState.create({
      doc: currentProps.value || "",
      extensions: [
        ...(useBasicSetup ? [basicSetup] : []),
        ...(currentProps.extensions || []),
        EditorView.updateListener.of((update) => {
          if (update.docChanged && currentProps.onChange) {
            currentProps.onChange(update.state.doc.toString());
          }
        }),
      ],
    });

    view = new EditorView({
      state,
      parent: container,
    });

    if (currentProps.autoFocus) {
      view.focus();
    }
  });

  // Handle value updates from props (one-way sync)
  createEffect(() => {
    const { value } = props();
    if (value !== undefined) {
      updateView(value);
    }
  });

  onCleanup(() => {
    if (view) {
      view.destroy();
      view = undefined;
    }
  });

  return {
    view: () => view,
  };
}