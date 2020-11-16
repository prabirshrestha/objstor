DROP TABLE IF EXISTS users;

CREATE TABLE users (
  uid       VARCHAR(256),                 -- user id
  uname     VARCHAR(256) NOT NULL UNIQUE, -- username
  upassword VARCHAR(256) NOT NULL,        -- password hash
  uemail    VARCHAR(500),                 -- user email
  ufname    VARCHAR(500),                 -- user friendly name
  uctime    DATETIME NOT NULL,            -- user created time
  umtime    DATETIME NOT NULL,            -- user modified time
  PRIMARY KEY (id)
);

DROP TABLE IF EXISTS objs;

CREATE TABLE objs (
  id        VARCHAR(256),
  iscol     INTEGER,            -- 1 for dir, 0 for file
  fmod      INTEGER,            -- file mod
  ctime     DATETIME NOT NULL,  -- created time
  mtime     DATETIME NOT NULL,  -- modified time
  utime     DATETIME NOT NULL,  -- updated time
  md5       VARCHAR(256),       -- md5 hash
  mime      VARCHAR(500),       -- mime
  name      TEXT NOT NULL,      -- name
  ext       TEXT NOT NULL,      -- extension
  text      TEXT,               -- text to full text search
  data      JSON,				-- extra data associated with data
  PRIMARY KEY (id)
);
