import { invoke } from "@tauri-apps/api/core";

type DispatchResponse<T> = {
  ok: boolean;
  data: T;
  error?: { message?: string };
};

export type WorkspaceCurrentResponse = {
  workspace_id: string;
  name: string;
  config_profile_id?: string | null;
  config_override?: Record<string, string> | null;
};

export type WorkspaceCurrentResponsePayload = {
  current: WorkspaceCurrentResponse | null;
};

export type WorkspaceDocumentNode = {
  id: string;
  folder_id: string;
  path: string;
  title: string;
  updated_at: number;
  ext?: string | null;
  lang?: string | null;
};

export type WorkspaceFolderNode = {
  id: string;
  root_path: string;
  documents: WorkspaceDocumentNode[];
};

export type WorkspaceFileTreeResponse = {
  workspace_id: string;
  folders: WorkspaceFolderNode[];
};

export type RenderDocumentResponse = {
  content: string | object; // content can be string (md/html) or object (markmap json)
};

export const fetchCurrentWorkspace = async () => {
  return invoke<DispatchResponse<WorkspaceCurrentResponsePayload>>("dispatch", {
    req: { command: "workspace_current", payload: {} }
  });
};

export const fetchWorkspaceFileTree = async (workspaceId: string) => {
  return invoke<DispatchResponse<WorkspaceFileTreeResponse>>("dispatch", {
    req: { command: "workspace_file_tree", payload: { workspace_id: workspaceId } }
  });
};

export const renderDocument = async (documentId: string, format: "markdown" | "html" | "markmap" = "markdown") => {
  return invoke<DispatchResponse<RenderDocumentResponse>>("dispatch", {
    req: { 
      command: "document_render", 
      payload: { 
        document_id: documentId,
        format 
      } 
    }
  });
};
