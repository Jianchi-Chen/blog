-- 0002_create_comments.sql
CREATE TABLE
    IF NOT EXISTS comments (
        comment_id TEXT NOT NULL PRIMARY KEY, -- 默认情况下 text的primary key并不会加上not null字段。其需要对应的struct字段为opiton<T>
        article_id TEXT,              -- 外键，指向文章表
        user TEXT,
        content TEXT,
        created_at TEXT
    );