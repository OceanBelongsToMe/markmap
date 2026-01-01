export type SashProps = {
  left: number;
  onDrag: (clientX: number) => void;
};

export const Sash = (props: SashProps) => {
  let sashRef: HTMLDivElement | undefined;

  const onPointerDown = (event: PointerEvent) => {
    sashRef?.setPointerCapture(event.pointerId);

    const onPointerMove = (moveEvent: PointerEvent) => {
      props.onDrag(moveEvent.clientX);
    };

    const onPointerUp = () => {
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp);
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
