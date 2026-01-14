import type { MarkmapAfterRender } from "./markmapRenderer";

export const createFitOnceAfterRender = (): MarkmapAfterRender => {
  let didFit = false;
  return ({ mm }) => {
    if (didFit) return;
    didFit = true;
    mm.fit();
  };
};
