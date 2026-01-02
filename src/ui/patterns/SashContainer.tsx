import { createMemo } from "solid-js";
import { Sash } from "../../layouts/Sash";
import { StableList } from "../components/StableList";

export type SashContainerProps = {
  sizes: number[];
  onDrag: (index: number, clientX: number) => void;
  onDragEnd?: (sizes: number[]) => void;
  class?: string;
  offsetLeft?: number;
};

export const SashContainer = (props: SashContainerProps) => {
  const positions = createMemo(() => {
    const result: number[] = [];
    let acc = 0;
    for (let i = 0; i < props.sizes.length - 1; i += 1) {
      acc += props.sizes[i];
      result.push(acc);
    }
    return result;
  });

  return (
    <div class={props.class ?? "sash-container"}>
      <StableList each={positions}>
        {(left, index) => (
          <Sash
            left={left() + (props.offsetLeft ?? 0)}
            onDrag={(x) => props.onDrag(index, x)}
            onDragEnd={() => props.onDragEnd?.(props.sizes)}
          />
        )}
      </StableList>
    </div>
  );
};
