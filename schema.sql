DROP TABLE IF EXISTS rank, watchlist;

CREATE TABLE rank (
  user_id BIGINT PRIMARY KEY,
  user_name TEXT NOT NULL,
  last_msg BIGINT NULL,
  points BIGINT NULL,
  level BIGINT NULL
);

CREATE TABLE watchlist (
  user_id BIGINT PRIMARY KEY,
  list TEXT NOT NULL
);