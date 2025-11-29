ALTER TABLE comments
ADD COLUMN parent_id TEXT;

ALTER TABLE comments
ADD COLUMN like_count INTEGER DEFAULT 0;

CREATE TABLE
    IF NOT EXISTS comment_likes (
        comment_id TEXT NOT NULL,
        user_id TEXT NOT NULL,
        created_at TEXT,
        PRIMARY KEY (comment_id, user_id)
    );