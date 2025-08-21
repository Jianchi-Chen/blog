-- 0002_create_users.sql
CREATE TABLE
    IF NOT EXISTS users (
        id TEXT PRIMARY KEY,
        username TEXT NOT NULL,
        password TEXT NOT NULL,
        token TEXT,
        identity TEXT NOT NULL DEFAULT 'vistor' -- 可约束为 ENUM 值
    );