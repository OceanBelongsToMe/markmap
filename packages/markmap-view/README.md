# markmap-view

![NPM](https://img.shields.io/npm/v/markmap-view.svg)
![License](https://img.shields.io/npm/l/markmap-view.svg)
![Downloads](https://img.shields.io/npm/dt/markmap-view.svg)

View markmaps in browser.

This package is separated from [markmap-lib](https://github.com/markmap/markmap/tree/master/packages/markmap-lib) to decrease the size of `node_modules` because you don't need this in `node_modules` most of the time.

👉 [Read the documentation](https://markmap.js.org/docs) for more detail.

## Inline editor adapter

`markmap-view` supports inline editing via an adapter interface. By default it uses
an embedded `contenteditable` editor. To integrate a rich editor (CodeMirror, Monaco, etc.),
provide an adapter through `editable.editor`.

```ts
import type { IInlineEditorAdapter, IEditorArgs } from "markmap-view";

const editorAdapter: IInlineEditorAdapter = {
  lockPointerEvents: true,
  open: (args: IEditorArgs) => {
    // mount your editor and call args.save / args.cancel
    return {
      close: (opts) => {
        if (opts?.cancel) args.cancel();
      },
    };
  },
};

const options = {
  editable: {
    enabled: true,
    editor: editorAdapter,
  },
};
```

### Example: switch to CodeMirror (app layer)

```ts
import type { IInlineEditorAdapter, IEditorArgs } from "markmap-view";

const editorAdapter: IInlineEditorAdapter = {
  lockPointerEvents: true,
  open: (args: IEditorArgs) => {
    // mount CodeMirror and call args.save / args.cancel
    return {
      close: (opts) => {
        if (opts?.cancel) args.cancel();
      },
    };
  },
};

const options = {
  editable: {
    enabled: true,
    editor: editorAdapter,
  },
};
```
