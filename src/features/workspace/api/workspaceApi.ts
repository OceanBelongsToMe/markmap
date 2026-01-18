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

export type MarkmapRootResponse = {
  content: object;
};

export type MarkmapChildrenResponse = {
  content: object[];
};

export type MarkmapNodeResponse = {
  content: object | null;
};

export type MarkmapEditMode = "node" | "subtree";

export type MarkmapEditMarkdownResponse = {
  content: string;
};

export type MarkmapResolvedAstNode = {
  kind: string;
  node_id?: string | null;
  text?: string | null;
  children: MarkmapResolvedAstNode[];
};

export type MarkmapResolvedAst = {
  root: MarkmapResolvedAstNode;
};

export type MarkmapAnchorKind = "block" | "inline";

export type MarkmapNodeIdAnchor = {
  kind: MarkmapAnchorKind;
  line?: number | null;
  from?: number | null;
  to?: number | null;
  node_id: string;
};

export type MarkmapEditAnchorsResponse = {
  anchors: MarkmapNodeIdAnchor[];
};

export type MarkmapResolvedAstTreeNode = {
  kind: string;
  node_id: string;
  children: MarkmapResolvedAstTreeNode[];
};

export type MarkmapResolvedAstTree = {
  root: MarkmapResolvedAstTreeNode;
};

export type MarkmapResolvedAstResponse = {
  ast: MarkmapResolvedAstTree;
};

export type WorkspaceAttachFolderRequest = {
  root_path: string;
  workspace_name?: string | null;
  workspace_id?: string | null;
  extensions?: string[] | null;
};

export type WorkspaceAttachFolderResponse = {
  workspace_id: string;
  folder_id: string;
  imported: number;
};

export type WorkspaceRecentFileRequest = {
  workspace_id: string;
  document_id: string;
  position: number;
};

export type WorkspaceRecentFileResponse = {
  workspace_id: string;
  document_id: string;
  last_opened_at: number;
  position: number;
};

export type WorkspaceRecentFilesRequest = {
  workspace_id: string;
};

export type WorkspaceRecentFilesResponse = {
  items: WorkspaceRecentFileResponse[];
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

export const fetchMarkmapRoot = async (documentId: string) => {
  return invoke<DispatchResponse<MarkmapRootResponse>>("dispatch", {
    req: {
      command: "markmap_get_root",
      payload: { document_id: documentId }
    }
  });
};

export const fetchMarkmapChildren = async (documentId: string, nodeId: string) => {
  return invoke<DispatchResponse<MarkmapChildrenResponse>>("dispatch", {
    req: {
      command: "markmap_get_children",
      payload: { document_id: documentId, node_id: nodeId }
    }
  });
};

export const fetchMarkmapNode = async (documentId: string, nodeId: string) => {
  return invoke<DispatchResponse<MarkmapNodeResponse>>("dispatch", {
    req: {
      command: "markmap_get_node",
      payload: { document_id: documentId, node_id: nodeId }
    }
  });
};

export const fetchMarkmapEditMarkdown = async (
  documentId: string,
  nodeId: string,
  mode: MarkmapEditMode
) => {
  return invoke<DispatchResponse<MarkmapEditMarkdownResponse>>("dispatch", {
    req: {
      command: "markmap_edit_get_markdown",
      payload: { document_id: documentId, node_id: nodeId, mode }
    }
  });
};

export const saveMarkmapEditMarkdown = async (
  documentId: string,
  nodeId: string,
  mode: MarkmapEditMode,
  content: string
) => {
  return invoke<DispatchResponse<Record<string, never>>>("dispatch", {
    req: {
      command: "markmap_edit_save_markdown",
      payload: { document_id: documentId, node_id: nodeId, mode, content }
    }
  });
};

export const applyMarkmapResolvedAst = async (
  documentId: string,
  rootNodeId: string,
  markdown: string,
  ast: MarkmapResolvedAst
) => {
  return invoke<DispatchResponse<Record<string, never>>>("dispatch", {
    req: {
      command: "markmap_edit_apply_resolved_ast",
      payload: { document_id: documentId, root_node_id: rootNodeId, markdown, ast }
    }
  });
};

export const fetchMarkmapEditAnchors = async (
  documentId: string,
  rootNodeId: string
) => {
  return invoke<DispatchResponse<MarkmapEditAnchorsResponse>>("dispatch", {
    req: {
      command: "markmap_edit_get_anchors",
      payload: { document_id: documentId, root_node_id: rootNodeId }
    }
  });
};

export const fetchMarkmapResolvedAst = async (
  documentId: string,
  rootNodeId: string
) => {
  return invoke<DispatchResponse<MarkmapResolvedAstResponse>>("dispatch", {
    req: {
      command: "markmap_edit_get_resolved_ast",
      payload: { document_id: documentId, root_node_id: rootNodeId }
    }
  });
};

export const attachFolder = async (payload: WorkspaceAttachFolderRequest) => {
  return invoke<DispatchResponse<WorkspaceAttachFolderResponse>>("dispatch", {
    req: { command: "workspace_attach_folder", payload }
  });
};

export const recordWorkspaceRecentFile = async (payload: WorkspaceRecentFileRequest) => {
  return invoke<DispatchResponse<WorkspaceRecentFileResponse>>("dispatch", {
    req: { command: "workspace_recent_file_record", payload }
  });
};

export const fetchWorkspaceRecentFiles = async (payload: WorkspaceRecentFilesRequest) => {
  return invoke<DispatchResponse<WorkspaceRecentFilesResponse>>("dispatch", {
    req: { command: "workspace_recent_files_list", payload }
  });
};
