import type { IInlineEditorAdapter, IInlineEditorSession, IEditorArgs } from '../../types';

type EditableState = {
  targetEl: HTMLElement;
  originalHtml: string;
  originalContentEditable: string | null;
  onKeyDown: (e: KeyboardEvent) => void;
  onBlur: () => void;
  stopEvent: (e: Event) => void;
};

function setupEditableTarget(host: HTMLDivElement) {
  let targetEl = host.querySelector('.mm-editable-text') as HTMLElement | null;
  if (!targetEl) {
    targetEl = document.createElement('span');
    targetEl.className = 'mm-editable-text';
    targetEl.textContent = host.textContent ?? '';
    host.replaceChildren(targetEl);
  }
  const originalHtml = targetEl.innerHTML;
  const originalContentEditable = targetEl.getAttribute('contenteditable');
  const initialText = targetEl.textContent ?? '';
  targetEl.textContent = initialText;
  targetEl.setAttribute('contenteditable', 'true');
  targetEl.classList.add('markmap-inline-editing');
  targetEl.focus();
  return { targetEl, originalHtml, originalContentEditable };
}

function placeCaretAtEnd(targetEl: HTMLElement) {
  const range = document.createRange();
  range.selectNodeContents(targetEl);
  range.collapse(false);
  const sel = window.getSelection();
  if (!sel) return;
  sel.removeAllRanges();
  sel.addRange(range);
}

function cleanupEditable(state: EditableState, restoreHtml: boolean) {
  const { targetEl, originalHtml, originalContentEditable, onKeyDown, onBlur, stopEvent } = state;
  targetEl.removeEventListener('keydown', onKeyDown);
  targetEl.removeEventListener('blur', onBlur);
  targetEl.removeEventListener('mousedown', stopEvent);
  targetEl.removeEventListener('dblclick', stopEvent);
  if (restoreHtml) {
    targetEl.innerHTML = originalHtml;
  }
  targetEl.classList.remove('markmap-inline-editing');
  if (originalContentEditable === null) {
    targetEl.removeAttribute('contenteditable');
  } else {
    targetEl.setAttribute('contenteditable', originalContentEditable);
  }
}

export function createContentEditableEditor(): IInlineEditorAdapter {
  return {
    open(args: IEditorArgs): IInlineEditorSession | void {
      const host = args.host;
      if (!host) return;
      const { targetEl, originalHtml, originalContentEditable } = setupEditableTarget(host);
      placeCaretAtEnd(targetEl);

      let disposed = false;
      const onKeyDown = (e: KeyboardEvent) => {
        if (e.key === 'Enter') {
          e.preventDefault();
          const text = targetEl.textContent ?? '';
          cleanupEditable(state, false);
          disposed = true;
          args.save(text);
        } else if (e.key === 'Escape') {
          e.preventDefault();
          cleanupEditable(state, true);
          disposed = true;
          args.cancel();
        }
      };

      const onBlur = () => {
        if (disposed) return;
        const text = targetEl.textContent ?? '';
        cleanupEditable(state, false);
        disposed = true;
        args.save(text);
      };

      const stopEvent = (e: Event) => {
        e.stopPropagation();
      };

      const state: EditableState = {
        targetEl,
        originalHtml,
        originalContentEditable,
        onKeyDown,
        onBlur,
        stopEvent,
      };

      targetEl.addEventListener('keydown', onKeyDown);
      targetEl.addEventListener('blur', onBlur);
      targetEl.addEventListener('mousedown', stopEvent);
      targetEl.addEventListener('dblclick', stopEvent);

      return {
        close: (opts) => {
          if (disposed) return;
          disposed = true;
          cleanupEditable(state, Boolean(opts?.cancel));
        },
      };
    },
  };
}
