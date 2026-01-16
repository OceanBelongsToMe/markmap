import { createSignal, onCleanup } from "solid-js";
import type { Accessor } from "solid-js";

export const useSidebarInteraction = (collapsed: Accessor<boolean>) => {
  const [isHovering, setIsHovering] = createSignal(false);
  const [isResizing, setIsResizing] = createSignal(false);
  let hideTimer: ReturnType<typeof setTimeout> | undefined;

  onCleanup(() => {
    if (hideTimer) clearTimeout(hideTimer);
  });

  const cancelHide = () => {
    if (hideTimer) {
      clearTimeout(hideTimer);
      hideTimer = undefined;
    }
  };

  const scheduleHide = () => {
    cancelHide();
    // 只有在折叠且非拖拽状态下才计划隐藏
    if (collapsed() && !isResizing()) {
      hideTimer = setTimeout(() => {
        setIsHovering(false);
      }, 300);
    }
  };

  const handlers = {
    onMouseEnter: () => {
      cancelHide();
      if (collapsed()) setIsHovering(true);
    },
    onMouseLeave: () => {
      scheduleHide();
    },
    onDragStart: () => {
      cancelHide();
      setIsResizing(true);
    },
    onDragEnd: () => {
      setIsResizing(false);
      // 这里的 Edge Case: 拖拽结束时如果在外部，目前无法自动隐藏，
      // 依赖用户下一次移动触发 Leave。这是已知且可接受的折衷。
    },
    // 提供给手动切换按钮使用，强制关闭悬停
    reset: () => {
      cancelHide();
      setIsHovering(false);
    }
  };

  return {
    isHovering,
    isResizing,
    handlers
  };
};
