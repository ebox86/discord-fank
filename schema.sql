--DROP TABLE IF EXISTS rank, watchlist, competitions, complist;

CREATE TABLE IF NOT EXISTS rank (
  user_id BIGINT PRIMARY KEY,
  user_name TEXT NOT NULL,
  last_msg BIGINT NULL,
  points BIGINT NULL,
  level BIGINT NULL
);

CREATE TABLE IF NOT EXISTS watchlist (
  user_id BIGINT PRIMARY KEY,
  list TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS complist (
  id serial PRIMARY KEY,
  user_id BIGINT NOT NULL,
  list TEXT NOT NULL,
  comp_id BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS competitions (
  id serial PRIMARY KEY,
  active BOOLEAN NOT NULL,
  reg_open BOOLEAN NOT NULL,
  start_date BIGINT NOT NULL,
  end_date BIGINT NOT NULL,
  name TEXT NOT NULL,
  winner BIGINT NULL
);