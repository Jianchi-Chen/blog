-- 0001_create_articles.sql
CREATE TABLE
    IF NOT EXISTS articles (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL UNIQUE,
        content TEXT,
        summary TEXT,
        created_at TEXT NOT NULL DEFAULT (datetime ('now')),
        update_at TEXT,
        status TEXT NOT NULL DEFAULT 'draft', -- 可约束为 ENUM 值
        views INTEGER NOT NULL DEFAULT 0,
        tags TEXT NOT NULL DEFAULT '[]' -- 存储为 JSON 字符串
    );