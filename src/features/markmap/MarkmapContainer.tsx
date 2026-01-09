import { Component, Show } from "solid-js";
import { MarkmapCanvas } from "../../ui/components/markmap/MarkmapCanvas";
import { useDocumentRender } from "../document/hooks/useDocumentRender";
import { useActiveDocument } from "../../state/workspace/useActiveDocument";
import { defaultOptions } from "markmap-view";

export type MarkmapContainerProps = {
  class?: string;
};

const MARKMAP_OPTIONS = {
  ...defaultOptions,
};

export const MarkmapContainer: Component<MarkmapContainerProps> = (props) => {
  const { activeDocId } = useActiveDocument();

  // Fetch markmap JSON
  const { data, loading, error } = useDocumentRender(activeDocId, () => "markmap");

  return (
    <div class={props.class}>
      <Show when={activeDocId()} fallback={
        <div class="flex items-center justify-center h-full text-gray-400">
          Select a file to view mind map
        </div>
      }>
        <Show when={!loading()} fallback={
          <div class="flex items-center justify-center h-full text-gray-400">
            Loading...
          </div>
        }>
          <Show when={data()}>
            <MarkmapCanvas data={data()} options={MARKMAP_OPTIONS} class="h-full" />
          </Show>
          <Show when={error()}>
            <div class="absolute top-0 left-0 right-0 bg-red-100 text-red-800 p-2 z-20">
              Error loading map: {error()?.message}
            </div>
          </Show>
        </Show>
      </Show>
    </div>
  );
};
