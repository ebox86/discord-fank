--DROP TABLE IF EXISTS rank, watchlist, competitions, complist;

CREATE TABLE IF NOT EXISTS rank (
  id serial PRIMARY KEY,
  guild_id BIGINT NOT NULL,
  user_id BIGINT NOT NULL,
  user_name TEXT NOT NULL,
  last_msg BIGINT NULL,
  points BIGINT NULL,
  level BIGINT NULL,
  UNIQUE (guild_id, user_id)
);

CREATE TABLE IF NOT EXISTS watchlist (
  id serial PRIMARY KEY,
  guild_id bigint NOT NULL,
  user_id BIGINT NOT NULL,
  list TEXT NOT NULL,
  UNIQUE (guild_id, user_id)
);

CREATE TABLE IF NOT EXISTS complist (
  id serial PRIMARY KEY,
  guild_id BIGINT NOT NULL,
  user_id BIGINT NOT NULL,
  list TEXT NOT NULL,
  comp_id BIGINT NOT NULL,
  UNIQUE (guild_id, user_id, comp_id)
);

CREATE TABLE IF NOT EXISTS competitions (
  id serial PRIMARY KEY,
  guild_id BIGINT NOT NULL,
  active BOOLEAN NOT NULL,
  reg_open BOOLEAN NOT NULL,
  start_date BIGINT NOT NULL,
  end_date BIGINT NOT NULL,
  name TEXT NOT NULL,
  winner BIGINT NULL,
  UNIQUE (guild_id, name)
);