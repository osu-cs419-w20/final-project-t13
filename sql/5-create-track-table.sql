CREATE TABLE IF NOT EXISTS track (
  id SERIAL NOT NULL,
  mbid TEXT UNIQUE NOT NULL,
  title TEXT NOT NULL,
  position integer NOT NULL,
  bit_rate integer NOT NULL,
  duration integer NOT NULL,
  file_location TEXT NOT NULL,
  album_id integer NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (album_id) REFERENCES album (id)
);

