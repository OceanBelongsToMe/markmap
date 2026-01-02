import { createSignal } from "solid-js";
import {
  fetchCurrentWorkspace,
  fetchWorkspaceFileTree,
  type WorkspaceCurrentResponse,
  type WorkspaceFileTreeResponse
} from "../../features/workspace/api/workspaceApi";

export type WorkspaceCurrent = {
  workspaceId: string;
  name: string;
  configProfileId?: string | null;
  configOverride?: Record<string, string> | null;
};

export type WorkspaceFileTree = {
  workspaceId: string;
  folders: WorkspaceFolderNode[];
};

export type WorkspaceFolderNode = {
  id: string;
  rootPath: string;
  documents: WorkspaceDocumentNode[];
};

export type WorkspaceDocumentNode = {
  id: string;
  folderId: string;
  path: string;
  title: string;
  updatedAt: number;
  ext?: string | null;
  lang?: string | null;
};

const [currentWorkspace, setCurrentWorkspace] = createSignal<WorkspaceCurrent | null>(null);
const [fileTree, setFileTree] = createSignal<WorkspaceFileTree | null>(null);
const [loading, setLoading] = createSignal(false);
const [error, setError] = createSignal<string | null>(null);
let inFlight: Promise<void> | null = null;

const mapCurrentWorkspace = (payload: WorkspaceCurrentResponse | null): WorkspaceCurrent | null => {
  if (!payload) return null;
  return {
    workspaceId: payload.workspace_id,
    name: payload.name,
    configProfileId: payload.config_profile_id ?? null,
    configOverride: payload.config_override ?? null
  };
};

const mapFileTree = (payload: WorkspaceFileTreeResponse): WorkspaceFileTree => {
  return {
    workspaceId: payload.workspace_id,
    folders: payload.folders.map((folder) => ({
      id: folder.id,
      rootPath: folder.root_path,
      documents: folder.documents.map((doc) => ({
        id: doc.id,
        folderId: doc.folder_id,
        path: doc.path,
        title: doc.title,
        updatedAt: doc.updated_at,
        ext: doc.ext ?? null,
        lang: doc.lang ?? null
      }))
    }))
  };
};

export const loadCurrentWorkspace = async () => {
  if (inFlight) {
    return inFlight;
  }
  inFlight = (async () => {
    setLoading(true);
    setError(null);
    try {
    const response = await fetchCurrentWorkspace();
    if (!response.ok) {
      throw new Error(response.error?.message ?? "加载工作空间失败");
    }
    const current = mapCurrentWorkspace(response.data.current);
    setCurrentWorkspace(current);
    if (current) {
      const tree = await fetchWorkspaceFileTree(current.workspaceId);
      if (!tree.ok) {
        throw new Error(tree.error?.message ?? "加载文件树失败");
      }
      setFileTree(mapFileTree(tree.data));
    } else {
      setFileTree(null);
    }
    } catch (err) {
      setError(err instanceof Error ? err.message : "加载工作空间失败");
    } finally {
      setLoading(false);
      inFlight = null;
    }
  })();
  return inFlight;
};

export const refreshWorkspaceTree = async (workspaceId?: string) => {
  const target = workspaceId ?? currentWorkspace()?.workspaceId;
  if (!target) {
    setFileTree(null);
    return;
  }
  setLoading(true);
  setError(null);
  try {
    const tree = await fetchWorkspaceFileTree(target);
    if (!tree.ok) {
      throw new Error(tree.error?.message ?? "加载文件树失败");
    }
    setFileTree(mapFileTree(tree.data));
  } catch (err) {
    setError(err instanceof Error ? err.message : "加载文件树失败");
  } finally {
    setLoading(false);
  }
};

export const useWorkspaceTreeState = () => {
  return {
    currentWorkspace,
    fileTree,
    loading,
    error
  };
};
