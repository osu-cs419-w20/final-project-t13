CREATE TABLE IF NOT EXISTS album (
  id SERIAL NOT NULL,
  mbid TEXT UNIQUE NOT NULL,
  title TEXT NOT NULL,
  image_url TEXT,
  artist_id integer NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (artist_id) REFERENCES artist (id) ON DELETE CASCADE
);

