export const taxonomy = {
  contentTypes: ["note", "outline", "markmap"],
  editorInputs: ["markdown-text", "wysiwyg", "markmap"],
  viewModes: ["markmap-preview"],
  tags: {
    system: ["important", "todo", "starred"],
    domain: ["product", "tech", "design"]
  }
} as const;
