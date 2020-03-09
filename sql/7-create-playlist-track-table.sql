CREATE TABLE IF NOT EXISTS playlist_track (
  playlist_id integer NOT NULL,
  track_id integer NOT NULL,
  position integer UNIQUE NOT NULL,
  PRIMARY KEY (playlist_id, position),
  FOREIGN KEY (playlist_id) REFERENCES playlist (id),
  FOREIGN KEY (track_id) REFERENCES track (id)
);
