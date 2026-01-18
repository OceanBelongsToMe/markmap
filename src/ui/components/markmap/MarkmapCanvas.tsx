import { createEffect, onCleanup } from "solid-js";
import "./markmap.css";
import { createFitOnceAfterRender } from "./markmapAfterRender";
import { createMarkmapRenderer, type MarkmapRenderPort } from "./markmapRenderer";

export type MarkmapCanvasProps = {
  data: any; // The JSON structure from backend
  options?: any; // Markmap options
  loader?: {
    loadChildren: (nodeId: string) => Promise<any[]>;
  };
  onRenderer?: (renderer: MarkmapRenderPort) => void;
  class?: string;
};

export const MarkmapCanvas = (props: MarkmapCanvasProps) => {
  let svgRef: SVGSVGElement | undefined;
  let hostRef: HTMLDivElement | undefined;
  const renderer = createMarkmapRenderer([createFitOnceAfterRender()]);

  createEffect(() => {
    if (!svgRef || !props.data) return;
    renderer.ensure(svgRef, props.options);
    renderer.setNodeLoader(props.loader);
    renderer.setData(props.data);
    props.onRenderer?.(renderer);
  });

  onCleanup(() => {
    renderer.destroy();
  });

  return (
    <div ref={hostRef} class={`markmap-container ${props.class || ""}`}>
      <svg ref={svgRef} class="markmap-svg" />
    </div>
  );
};
