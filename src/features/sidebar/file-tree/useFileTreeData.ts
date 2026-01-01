export type FileTreeNode = {
  id: string;
  label: string;
  children?: FileTreeNode[];
};

export const useFileTreeData = () => {
  const data: FileTreeNode[] = [
    { id: "root", label: "Workspace", children: [] }
  ];

  return {
    data
  };
};
