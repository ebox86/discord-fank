DROP TABLE IF EXISTS rank;

CREATE TABLE rank (
  user_id BIGINT PRIMARY KEY,
  user_name TEXT NOT NULL,
  last_msg BIGINT NULL,
  points BIGINT NULL,
  level BIGINT NULL
);