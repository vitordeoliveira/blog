ALTER TABLE posts ADD COLUMN created_at TEXT;
ALTER TABLE posts ADD COLUMN updated_at TEXT;

CREATE TABLE new_posts (
    id varchar(255) PRIMARY KEY,
    views INT DEFAULT 0,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO new_posts (id, views, created_at, updated_at)
SELECT id, views, 
       COALESCE(created_at, CURRENT_TIMESTAMP), 
       COALESCE(updated_at, CURRENT_TIMESTAMP)
FROM posts;

DROP TABLE posts;

ALTER TABLE new_posts RENAME TO posts;
