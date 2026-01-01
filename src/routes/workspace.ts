import type { RouteDefinition } from "./types";
import { WorkspacePage } from "../pages/WorkspacePage";

export const workspaceRoutes: RouteDefinition[] = [
  { path: "/", component: WorkspacePage }
];
