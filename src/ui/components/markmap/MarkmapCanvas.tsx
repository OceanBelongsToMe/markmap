import { createEffect, onCleanup } from "solid-js";
import "./markmap.css";
import { createFitOnceAfterRender } from "./markmapAfterRender";
import { createMarkmapRenderer } from "./markmapRenderer";

export type MarkmapCanvasProps = {
  data: any; // The JSON structure from backend
  options?: any; // Markmap options
  class?: string;
};

export const MarkmapCanvas = (props: MarkmapCanvasProps) => {
  let svgRef: SVGSVGElement | undefined;
  const renderer = createMarkmapRenderer([createFitOnceAfterRender()]);

  createEffect(() => {
    if (!svgRef || !props.data) return;
    renderer.ensure(svgRef, props.options);
    renderer.setData(props.data);
  });

  onCleanup(() => {
    renderer.destroy();
  });

  return (
    <div class={`markmap-container ${props.class || ""}`}>
      <svg ref={svgRef} class="markmap-svg" />
    </div>
  );
};
