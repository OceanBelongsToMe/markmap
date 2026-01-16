export type SashProps = {
  left: number;
  onDrag: (clientX: number) => void;
  onDragStart?: () => void;
  onDragEnd?: () => void;
};

export const Sash = (props: SashProps) => {
  let sashRef: HTMLDivElement | undefined;

  const onPointerDown = (event: PointerEvent) => {
    event.preventDefault();
    sashRef?.setPointerCapture(event.pointerId);
    document.documentElement.dataset.resizing = "true";
    props.onDragStart?.();

    // Calculate the initial offset relative to the logical center (props.left)
    // This ensures that clicking anywhere on the sash (even with visual transforms) doesn't cause a jump
    const initialOffset = event.clientX - props.left;

    const onPointerMove = (moveEvent: PointerEvent) => {
      // Adjust the reported clientX by the initial offset
      props.onDrag(moveEvent.clientX - initialOffset);
    };

    const onPointerUp = () => {
      window.removeEventListener("pointermove", onPointerMove);
      delete document.documentElement.dataset.resizing;
      props.onDragEnd?.();
    };

    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp, { once: true });
  };

  return (
    <div
      class="sash"
      ref={sashRef}
      style={{ left: `${props.left}px` }}
      onPointerDown={onPointerDown}
      role="separator"
      aria-orientation="vertical"
    />
  );
};
