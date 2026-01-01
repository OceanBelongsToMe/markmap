PRAGMA foreign_keys = ON;

ALTER TABLE node_link
ADD COLUMN link_type TEXT NOT NULL DEFAULT 'Inline';
