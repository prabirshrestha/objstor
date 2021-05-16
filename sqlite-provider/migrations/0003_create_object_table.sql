CREATE TABLE IF NOT EXISTS object (
    id varchar(256) PRIMARY KEY,
    parent_id varchar(256),
    storage_id varchar(256) NOT NULL,
    name TEXT NOT NULL,
    type ObjectType NOT NULL CHECK (type IN (0,1)), -- 0:file, 1:dir
    ctime DATETIME,
    mtime DATETIME,
    md5 varchar(32),
    sha256 varchar(256),
    description TEXT,                               -- full text search
    data JSON,                                      -- extra metadata
    FOREIGN KEY (parent_id) REFERENCES objects (id),
    FOREIGN KEY (storage_id) REFERENCES storage (id)
)
