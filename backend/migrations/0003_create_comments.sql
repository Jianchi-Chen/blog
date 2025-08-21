-- 0002_create_comments.sql
CREATE TABLE
    IF NOT EXISTS comments (
        id TEXT PRIMARY KEY,
        user TEXT NOT NULL,
        content TEXT NOT NULL,
        created_at TEXT
    );