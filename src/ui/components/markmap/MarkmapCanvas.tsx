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
    // Deep clone data to break reactivity connection and prevent mutations from triggering updates
    const data = props.data ? structuredClone(props.data) : null;
    
    if (!svgRef || !data) return;

    if (!mm) {
      mm = Markmap.create(svgRef, props.options, data);
    } else {
      mm.setData(data);
      mm.fit(); // Still good to call fit() on data change
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
