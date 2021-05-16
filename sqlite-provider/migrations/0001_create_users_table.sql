CREATE TABLE IF NOT EXISTS user (
    id varchar(256) PRIMARY KEY,
    username varchar(256) UNIQUE NOT NULL,
    password varchar(256) NOT NULL,
    /* created DATETIME NOT NULL, */
    is_admin BOOLEAN NOT NULL CHECK (is_admin IN (0,1)),
    is_locked BOOLEAN NOT NULL CHECK (is_locked IN (0,1))
)
