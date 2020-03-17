CREATE TABLE IF NOT EXISTS album_image (
  album_id integer NOT NULL,
  url TEXT NOT NULL,
  PRIMARY KEY (album_id, url),
  FOREIGN KEY (album_id) REFERENCES album (id)
);

