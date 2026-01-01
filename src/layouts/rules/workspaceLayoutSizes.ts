export const workspacePaneSizes = {
  sidebar: { initialPx: 240, minPx: 180 },
  editor: { initialPx: 700, minPx: 480, maxPx: 1200 },
  preview: { initialPx: 500, minPx: 300, maxPx: 1000 }
};

export const workspaceLayoutMins = {
  sideMin: workspacePaneSizes.sidebar.minPx,
  editorMin: workspacePaneSizes.editor.minPx,
  previewMin: workspacePaneSizes.preview.minPx
};
