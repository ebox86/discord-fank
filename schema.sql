--DROP TABLE IF EXISTS rank, watchlist, competition, complist, guild, account, task;

CREATE TABLE IF NOT EXISTS guild (
  id BIGINT PRIMARY KEY,
  name VARCHAR
);

CREATE TABLE IF NOT EXISTS account (
  id BIGINT PRIMARY KEY,
  name VARCHAR
);

CREATE TABLE IF NOT EXISTS rank (
  id serial PRIMARY KEY,
  guild_id BIGINT,
  user_id BIGINT,
  xp BIGINT,
  level BIGINT,
  rank BIGINT,
  UNIQUE (guild_id, user_id),
  CONSTRAINT fk_guild_id FOREIGN KEY (guild_id) REFERENCES guild (id),
  CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES account (id)
);

CREATE TABLE IF NOT EXISTS watchlist (
  id serial PRIMARY KEY,
  guild_id BIGINT,
  user_id BIGINT,
  list VARCHAR,
  UNIQUE (guild_id, user_id),
  CONSTRAINT fk_guild_id FOREIGN KEY (guild_id) REFERENCES guild (id),
  CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES account (id)
);

CREATE TABLE IF NOT EXISTS competition (
  id serial PRIMARY KEY,
  name VARCHAR,
  guild_id BIGINT,
  active BOOLEAN,
  reg_open BOOLEAN,
  image VARCHAR,
  color VARCHAR,
  start_date BIGINT,
  end_date BIGINT,
  winner BIGINT,
  UNIQUE (guild_id, id),
  CONSTRAINT fk_guild_id FOREIGN KEY (guild_id) REFERENCES guild (id),
  CONSTRAINT fk_winner FOREIGN KEY (winner) REFERENCES account (id)
);

CREATE TABLE IF NOT EXISTS complist (
  id serial PRIMARY KEY,
  guild_id BIGINT,
  user_id BIGINT,
  list VARCHAR,
  comp_id BIGINT,
  UNIQUE (guild_id, user_id, comp_id),
  CONSTRAINT fk_guild_id FOREIGN KEY (guild_id) REFERENCES guild (id),
  CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES account (id),
  CONSTRAINT fk_comp_id FOREIGN KEY (comp_id) REFERENCES competition (id)
);

CREATE TABLE IF NOT EXISTS task (
  id serial PRIMARY KEY,
  guild_id BIGINT,
  channel_id BIGINT,
  type VARCHAR,
  content VARCHAR,
  start_date BIGINT ,
  end_date BIGINT,
  UNIQUE (guild_id, id),
  CONSTRAINT fk_guild_id FOREIGN KEY (guild_id) REFERENCES guild (id)
);

CREATE TABLE IF NOT EXISTS message (
  id serial PRIMARY KEY,
  guild_id BIGINT,
  channel_id BIGINT,
  user_id BIGINT,
  timestamp BIGINT,
  UNIQUE (guild_id, id),
  CONSTRAINT fk_guild_id FOREIGN KEY (guild_id) REFERENCES guild (id),
  CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES account (id)
);