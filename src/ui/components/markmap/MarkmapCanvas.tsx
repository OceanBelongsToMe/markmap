import { createEffect, onCleanup } from "solid-js";
import { Markmap } from "markmap-view";
import "./markmap.css";

export type MarkmapCanvasProps = {
  data: any; // The JSON structure from backend
  options?: any; // Markmap options
  class?: string;
};

export const MarkmapCanvas = (props: MarkmapCanvasProps) => {
  let svgRef: SVGSVGElement | undefined;
  let mm: Markmap | undefined;

  createEffect(() => {
    if (!svgRef || !props.data) return;
    console.log(props.data);
    if (!mm) {
      mm = Markmap.create(svgRef, props.options, props.data);
    } else {
      mm.setData(props.data, props.options);
    }
  });

  onCleanup(() => {
    if (mm) {
      mm.destroy();
      mm = undefined;
    }
  });

  return (
    <div class={`markmap-container ${props.class || ""}`}>
      <svg ref={svgRef} class="markmap-svg" />
    </div>
  );
};
