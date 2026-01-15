import type { RecentFileEntry } from "./mapRecentFilesToEntries";

export type RecentFileGroup = {
  id: "today" | "week" | "older";
  labelKey: "recentToday" | "recentWeek" | "recentOlder";
  entries: RecentFileEntry[];
};

const startOfToday = () => {
  const now = new Date();
  now.setHours(0, 0, 0, 0);
  return now.getTime();
};

export const groupRecentFilesByTime = (
  entries: RecentFileEntry[]
): RecentFileGroup[] => {
  const todayStart = startOfToday();
  const weekStart = todayStart - 7 * 24 * 60 * 60 * 1000;

  const groups: Record<RecentFileGroup["id"], RecentFileGroup> = {
    today: { id: "today", labelKey: "recentToday", entries: [] },
    week: { id: "week", labelKey: "recentWeek", entries: [] },
    older: { id: "older", labelKey: "recentOlder", entries: [] }
  };

  for (const entry of entries) {
    if (entry.lastOpenedAt >= todayStart) {
      groups.today.entries.push(entry);
    } else if (entry.lastOpenedAt >= weekStart) {
      groups.week.entries.push(entry);
    } else {
      groups.older.entries.push(entry);
    }
  }

  return Object.values(groups).filter((group) => group.entries.length > 0);
};
