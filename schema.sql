DROP TABLE IF EXISTS rank, watchlist, competitions, complist;

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

CREATE TABLE complist (
  id serial PRIMARY KEY,
  user_id BIGINT NOT NULL,
  list TEXT NOT NULL,
  comp_id BIGINT NOT NULL
);

CREATE TABLE competitions (
  id serial PRIMARY KEY,
  active BOOLEAN NOT NULL,
  start_date BIGINT NOT NULL,
  end_date BIGINT NOT NULL,
  name TEXT NOT NULL,
  winner BIGINT NULL
);