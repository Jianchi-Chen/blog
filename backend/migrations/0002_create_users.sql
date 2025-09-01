-- 0002_create_users.sql
CREATE TABLE
    IF NOT EXISTS users (
        id TEXT PRIMARY KEY,
        username TEXT,
        password TEXT,
        token TEXT,
        identity TEXT DEFAULT 'visitor' -- 可约束为 ENUM 值
    );