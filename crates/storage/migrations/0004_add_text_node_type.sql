INSERT INTO node_types (id, name) VALUES
    (32, 'Text')
ON CONFLICT(id) DO NOTHING;
