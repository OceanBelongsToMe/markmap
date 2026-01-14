import type { INode } from "markmap-common";

const HEADING_ICON_HTML_BY_LEVEL: Record<number, string> = {
  1: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
    <path d="M12 18V6"></path>
    <path d="m17 12 3-2v8"></path>
  </svg>
</span>
`,
  2: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
    <path d="M12 18V6"></path>
    <path d="M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1"></path>
  </svg>
</span>
`,
  3: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
    <path d="M12 18V6"></path>
    <path d="M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2"></path>
    <path d="M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2"></path>
  </svg>
</span>
`,
  4: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M12 18V6"></path>
    <path d="M17 10v3a1 1 0 0 0 1 1h3"></path>
    <path d="M21 10v8"></path>
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
  </svg>
</span>
`,
  5: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
    <path d="M12 18V6"></path>
    <path d="M17 13v-3h4"></path>
    <path d="M17 17.7c.4.2.8.3 1.3.3 1.5 0 2.7-1.1 2.7-2.5S19.8 13 18.3 13H17"></path>
  </svg>
</span>
`,
  6: `
<span class="mm-heading-icon" aria-hidden="true">
  <svg class="mm-heading-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor"
    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M4 12h8"></path>
    <path d="M4 18V6"></path>
    <path d="M12 18V6"></path>
    <circle cx="19" cy="16" r="2"></circle>
    <path d="M20 10c-2 2-3 3.5-3 6"></path>
  </svg>
</span>
`,
};

const getHeadingIconHtml = (level: number | undefined) => {
  if (!level) return "";
  return HEADING_ICON_HTML_BY_LEVEL[level] || "";
};

export const nodeContentWithHeadingIcons = (node: INode): string => {
  const level = Number(node?.payload?.heading_level || 0);
  const iconHtml = getHeadingIconHtml(level);
  if (!iconHtml) return node.content;
  return `${iconHtml}<span class="mm-heading-text">${node.content}</span>`;
};
