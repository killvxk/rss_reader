-- Feeds 表
CREATE TABLE feeds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    url TEXT UNIQUE NOT NULL,
    category TEXT NOT NULL,
    last_fetched TIMESTAMP,
    fetch_error TEXT
);

-- Articles 表
CREATE TABLE articles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    feed_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    link TEXT UNIQUE NOT NULL,
    content TEXT,
    published TIMESTAMP NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT 0,
    is_bookmarked BOOLEAN NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE
);

-- 全文搜索索引
CREATE VIRTUAL TABLE articles_fts USING fts5(
    title,
    content,
    content=articles,
    content_rowid=id
);

-- FTS 触发器
CREATE TRIGGER articles_ai AFTER INSERT ON articles BEGIN
    INSERT INTO articles_fts(rowid, title, content)
    VALUES (new.id, new.title, new.content);
END;

CREATE TRIGGER articles_ad AFTER DELETE ON articles BEGIN
    DELETE FROM articles_fts WHERE rowid = old.id;
END;

CREATE TRIGGER articles_au AFTER UPDATE ON articles BEGIN
    UPDATE articles_fts SET title = new.title, content = new.content
    WHERE rowid = new.id;
END;

-- Tags 表
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);

-- Article-Tag 关联表
CREATE TABLE article_tags (
    article_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (article_id, tag_id),
    FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- 索引
CREATE INDEX idx_articles_feed_id ON articles(feed_id);
CREATE INDEX idx_articles_published ON articles(published DESC);
CREATE INDEX idx_articles_is_read ON articles(is_read);
CREATE INDEX idx_articles_is_bookmarked ON articles(is_bookmarked);
