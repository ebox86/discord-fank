DROP TABLE IF EXISTS rank;

CREATE TABLE rank (
  user_id BIGINT PRIMARY KEY,
  user_name TEXT NOT NULL,
  last_msg BIGINT NULL,
  msg_count BIGINT NULL,
  level BIGINT NULL
);