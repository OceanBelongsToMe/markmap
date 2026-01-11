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
  let didFit = false;

  createEffect(() => {
    if (!svgRef || !props.data) return;
    if (!mm) {
      mm = Markmap.create(svgRef, props.options);
    } else if (props.options) {
      mm.setOptions(props.options);
    }

    (mm as any).state.data = props.data;
    mm.updateStyle();
    mm.renderData().then(() => {
      if (!didFit) {
        didFit = true;
        mm?.fit();
      }
    });
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
