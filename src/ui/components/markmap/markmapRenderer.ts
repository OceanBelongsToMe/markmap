import { Markmap, type INodeLoader } from "markmap-view";
import { wrapFunction } from "markmap-common";

export type MarkmapAfterRenderContext = {
  mm: Markmap;
};

export type MarkmapAfterRender = (
  ctx: MarkmapAfterRenderContext
) => void | Promise<void>;

const RENDER_DATA_ORIGINAL = Symbol("markmap-renderData-original");

type MarkmapRenderer = {
  ensure: (svg: SVGSVGElement, options?: any) => Markmap;
  setData: (data: any) => void;
  setNodeLoader: (loader?: INodeLoader) => void;
  destroy: () => void;
};

export const createMarkmapRenderer = (
  afterRender: MarkmapAfterRender[] = []
): MarkmapRenderer => {
  let mm: Markmap | undefined;

  const runAfterRender = async (instance: Markmap) => {
    for (const hook of afterRender) {
      await hook({ mm: instance });
    }
  };

  const wrapRenderData = (instance: Markmap) => {
    const anyMm = instance as any;
    if (anyMm[RENDER_DATA_ORIGINAL]) return;
    const original = instance.renderData.bind(instance);
    anyMm[RENDER_DATA_ORIGINAL] = original;
    instance.renderData = wrapFunction(original, async (fn, ...args) => {
      const result = await fn(...args);
      await runAfterRender(instance);
      return result;
    });
  };

  const restoreRenderData = (instance: Markmap) => {
    const anyMm = instance as any;
    const original = anyMm[RENDER_DATA_ORIGINAL];
    if (original) {
      instance.renderData = original;
      delete anyMm[RENDER_DATA_ORIGINAL];
    }
  };

  return {
    ensure(svg, options) {
      if (!mm) {
        mm = Markmap.create(svg, options);
        wrapRenderData(mm);
      } else if (options) {
        mm.setOptions(options);
      }
      return mm;
    },
    setData(data) {
      if (!mm) return;
      (mm as any).state.data = data;
      mm.updateStyle();
      mm.renderData();
    },
    setNodeLoader(loader) {
      if (!mm) return;
      if (typeof (mm as any).setNodeLoader === "function") {
        (mm as any).setNodeLoader(loader);
      }
    },
    destroy() {
      if (!mm) return;
      restoreRenderData(mm);
      mm.destroy();
      mm = undefined;
    },
  };
};
