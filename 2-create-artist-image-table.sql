CREATE TABLE IF NOT EXISTS artist_image (
  artist_id integer NOT NULL,
  url TEXT NOT NULL,
  PRIMARY KEY (artist_id, url),
  FOREIGN KEY (artist_id) REFERENCES artist (id)
);

