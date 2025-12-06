-- 0001_create_articles.sql
CREATE TABLE
    IF NOT EXISTS articles (
        id TEXT PRIMARY KEY,
        title TEXT,
        content TEXT,
        summary TEXT,
        created_at TEXT DEFAULT (datetime ('now')),
        update_at TEXT,
        status TEXT DEFAULT 'draft', -- 可约束为 ENUM 值
        views INTEGER DEFAULT 0,
        tags TEXT -- 存储为 JSON 字符串
    );