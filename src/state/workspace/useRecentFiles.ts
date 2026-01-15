import { createRoot, createSignal } from "solid-js";
import {
  fetchWorkspaceRecentFiles,
  recordWorkspaceRecentFile,
  type WorkspaceRecentFileResponse
} from "../../features/workspace/api/workspaceApi";

export type WorkspaceRecentFile = {
  workspaceId: string;
  documentId: string;
  lastOpenedAt: number;
  position: number;
};

const mapRecentFile = (item: WorkspaceRecentFileResponse): WorkspaceRecentFile => {
  return {
    workspaceId: item.workspace_id,
    documentId: item.document_id,
    lastOpenedAt: item.last_opened_at,
    position: item.position
  };
};

const sortRecentFiles = (items: WorkspaceRecentFile[]) => {
  return [...items].sort((a, b) => b.lastOpenedAt - a.lastOpenedAt);
};

const root = createRoot(() => {
  const [items, setItems] = createSignal<WorkspaceRecentFile[]>([]);
  const [page, setPage] = createSignal(1);
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);
  const [hasMore, setHasMore] = createSignal(false);
  const pageSize = 20;
  let inFlight: Promise<void> | null = null;
  let allItems: WorkspaceRecentFile[] = [];
  let lastWorkspaceId: string | null = null;

  const applyPage = (nextPage: number) => {
    const limit = nextPage * pageSize;
    const slice = allItems.slice(0, limit);
    setItems(slice);
    setPage(nextPage);
    setHasMore(allItems.length > slice.length);
  };

  const load = async (workspaceId?: string | null, opts?: { force?: boolean }) => {
    if (!workspaceId) {
      allItems = [];
      lastWorkspaceId = null;
      setItems([]);
      setPage(1);
      setHasMore(false);
      setError(null);
      return;
    }

    if (!opts?.force && lastWorkspaceId === workspaceId && allItems.length > 0) {
      applyPage(page());
      return;
    }

    if (inFlight) return inFlight;
    inFlight = (async () => {
      setLoading(true);
      setError(null);
      try {
        const response = await fetchWorkspaceRecentFiles({ workspace_id: workspaceId });
        if (!response.ok) {
          throw new Error(response.error?.message ?? "加载最近文件失败");
        }
        allItems = sortRecentFiles(response.data.items.map(mapRecentFile));
        lastWorkspaceId = workspaceId;
        applyPage(1);
      } catch (err) {
        setError(err instanceof Error ? err.message : "加载最近文件失败");
      } finally {
        setLoading(false);
        inFlight = null;
      }
    })();
    return inFlight;
  };

  const loadMore = () => {
    if (loading() || !hasMore()) return;
    applyPage(page() + 1);
  };

  const recordOpen = async (
    workspaceId: string,
    documentId: string,
    openedAt?: number
  ) => {
    if (!workspaceId || !documentId) return;
    const now = openedAt ?? Date.now();
    const existingIndex = allItems.findIndex((item) => item.documentId === documentId);
    if (existingIndex >= 0) {
      const existing = { ...allItems[existingIndex], lastOpenedAt: now, position: 0 };
      allItems.splice(existingIndex, 1);
      allItems.unshift(existing);
    } else {
      allItems.unshift({
        workspaceId,
        documentId,
        lastOpenedAt: now,
        position: 0
      });
    }
    applyPage(page());
    void recordWorkspaceRecentFile({
      workspace_id: workspaceId,
      document_id: documentId,
      position: 0
    });
  };

  return {
    items,
    page,
    pageSize: () => pageSize,
    loading,
    error,
    hasMore,
    load,
    loadMore,
    recordOpen
  };
});

export const useRecentFiles = () => root;
