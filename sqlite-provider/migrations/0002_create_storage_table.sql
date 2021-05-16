CREATE TABLE IF NOT EXISTS storage (
    id varchar(256) PRIMARY KEY,
    name varchar(500) NOT NULL,
    created DATETIME NOT NULL,
    provider varchar(256) NOT NULL,
    data TEXT,
    edata TEXT,
    is_locked BOOLEAN NOT NULL CHECK (is_locked IN (0,1)),
    is_indexed BOOLEAN NOT NULL CHECK (is_indexed IN (0,1))
)
