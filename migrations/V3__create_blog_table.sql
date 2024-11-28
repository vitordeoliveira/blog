PRAGMA foreign_keys = ON;

CREATE TABLE blogs (
  id varchar(36) PRIMARY KEY,
  user_id varchar(36),
  FOREIGN key (user_id) REFERENCES User(id)
);
