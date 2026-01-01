import { labels } from "./labels";

export type NavItem = {
  id: string;
  labelKey: keyof typeof labels;
  path: string;
  children?: NavItem[];
};

export const navModel: NavItem[] = [
  { id: "workspace", labelKey: "workspace", path: "/workspace" },
  {
    id: "files",
    labelKey: "files",
    path: "/files",
    children: [
      { id: "tree", labelKey: "tree", path: "/files/tree" },
      { id: "recent", labelKey: "recent", path: "/files/recent" },
      { id: "favorites", labelKey: "favorites", path: "/files/favorites" }
    ]
  },
  { id: "tags", labelKey: "tags", path: "/tags" },
  { id: "search", labelKey: "search", path: "/search" },
  { id: "settings", labelKey: "settings", path: "/settings" }
];
