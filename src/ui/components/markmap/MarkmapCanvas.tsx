import { createEffect, onCleanup } from "solid-js";
import { Markmap } from "markmap-view";
import "./markmap.css";

const HEADING_ICON_HTML_BY_LEVEL: Record<number, string> = {
  1: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
    <path d="M12 18V6"></path>
    <path d="m17 12 3-2v8"></path>
  </svg>
</span>
`,
  2: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
    <path d="M12 18V6"></path>
    <path d="M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1"></path>
  </svg>
</span>
`,
  3: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
    <path d="M12 18V6"></path>
    <path d="M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2"></path>
    <path d="M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2"></path>
  </svg>
</span>
`,
  4: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M12 18V6"></path>
    <path d="M17 10v3a1 1 0 0 0 1 1h3"></path>
    <path d="M21 10v8"></path>
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
  </svg>
</span>
`,
  5: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
    <path d="M12 18V6"></path>
    <path d="M17 13v-3h4"></path>
    <path d="M17 17.7c.4.2.8.3 1.3.3 1.5 0 2.7-1.1 2.7-2.5S19.8 13 18.3 13H17"></path>
  </svg>
</span>
`,
  6: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
    <path d="M12 18V6"></path>
    <circle cx="19" cy="16" r="2"></circle>
    <path d="M20 10c-2 2-3 3.5-3 6"></path>
  </svg>
</span>
`,
};

const getHeadingIconHtml = (level: number | undefined) => {
  if (!level) return "";
  return HEADING_ICON_HTML_BY_LEVEL[level] || "";
};

const injectHeadingIcons = (mm: Markmap) => {
  const g = (mm as any)?.g;
  if (!g) return;

  g.selectAll("g.markmap-node").each(function (this: SVGGElement, d: any) {
    const host = (this as SVGGElement).querySelector("foreignObject div div");
    if (!host) return;
    const hostEl = host as HTMLElement;

    const level = Number(d?.payload?.heading_level || 0);
    const iconHtml = getHeadingIconHtml(level);
    const isHeading = Boolean(iconHtml);
    const existing = host.querySelector(".mm-heading-icon");
    const currentLevel = Number(hostEl.dataset.mmHeadingLevel || 0);

    if (isHeading) {
      if (!existing || currentLevel !== level) {
        const textWrap = host.querySelector(".mm-heading-text");
        const original = textWrap ? textWrap.innerHTML : host.innerHTML;
        host.innerHTML = `${iconHtml}<span class="mm-heading-text">${original}</span>`;
        hostEl.dataset.mmHeadingLevel = String(level);
      }
    } else if (existing) {
      const textWrap = host.querySelector(".mm-heading-text");
      host.innerHTML = textWrap ? textWrap.innerHTML : host.innerHTML;
      hostEl.dataset.mmHeadingLevel = "";
    }
  });
};

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
      injectHeadingIcons(mm as Markmap);
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
