PRAGMA foreign_keys = ON;

ALTER TABLE node_link
ADD COLUMN ref_id TEXT;
