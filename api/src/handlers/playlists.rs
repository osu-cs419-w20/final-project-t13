use std::collections::BTreeSet;

use warp::http::StatusCode;

use crate::Error;
use crate::db::DB;
use crate::filters::RelationsOption;

#[derive(Serialize)]
pub struct Playlist {
    id: i32,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracks: Option<Vec<PlaylistTrack>>,
}

#[derive(Serialize)]
pub struct PlaylistTrack {
    track: super::tracks::Track,
    position: i32,
}

#[derive(Deserialize)]
pub struct NewPlaylist {
    name: String,
}

#[derive(Deserialize)]
pub struct AddToPlaylist {
    track_id: i32,
}

pub async fn get_playlists(db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let client = db.get().await?;
    let stmt = client.prepare("
        SELECT id, name
        FROM playlist
        ORDER BY name ASC
    ").await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[]).await.map_err(Error::from)?;
    let mut playlists = Vec::new();

    for row in rows {
        let playlist = Playlist {
            id: row.get(0),
            name: row.get(1),
            tracks: None,
        };
        playlists.push(playlist);
    }

    Ok(warp::reply::json(&playlists))
}

pub async fn create_playlist(p: NewPlaylist, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let client = db.get().await?;
    let stmt = client.prepare("
        INSERT INTO playlist (name)
        VALUES ($1)
        RETURNING id, name
    ").await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&p.name]).await.map_err(Error::from)?;

    if rows.is_empty() {
        return Err(warp::reject());
    }

    let row = &rows[0];
    let playlist = Playlist {
        id: row.get(0),
        name: row.get(1),
        tracks: None,
    };

    Ok(warp::reply::json(&playlist))
}

pub async fn get_playlist_with_id(id: i32, rels: RelationsOption, db: DB) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let rels = rels.relations.unwrap_or(BTreeSet::new());
    let mut select_fields = vec!["P.id", "P.name"];
    let mut joins = Vec::new();

    let mut loading_tracks = false;

    for rel in rels {
        use crate::filters::Relation;
        match rel {
            Relation::Tracks => {
                select_fields.extend_from_slice(&[
                    "T.id",
                    "T.mbid",
                    "T.title",
                    "T.position",
                    "T.bit_rate",
                    "T.duration",
                    "T.file_location",
                    "T.album_id",
                    "PT.position AS playlist_position",
                ]);
                joins.extend_from_slice(&[
                    "INNER JOIN playlist_track PT ON PT.playlist_id = P.id",
                    "INNER JOIN track T ON T.id = PT.track_id",
                ]);
                loading_tracks = true;
            }
            _ => {}
        }
    }

    let select_fields = select_fields.as_slice().join(", ");
    let joins = joins.as_slice().join("\n");
    let q = format!("SELECT {} FROM playlist P {} WHERE P.id = $1", select_fields, joins);
    let client = db.get().await?;
    let stmt = client.prepare(&q).await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&id]).await.map_err(Error::from)?;

    if rows.is_empty() {
        return Ok(Box::new(warp::reply::with_status(warp::reply(), StatusCode::NOT_FOUND)));
    }

    let mut tracks = None;

    if loading_tracks {
        let mut playlist_tracks = Vec::new();
        for row in &rows {
            let track = super::tracks::Track {
                id: row.get(2),
                mbid: row.get(3),
                title: row.get(4),
                position: row.get(5),
                bit_rate: row.get(6),
                duration: row.get(7),
                file_location: row.get(8),
                album_id: row.get(9),
            };
            let pt = PlaylistTrack {
                track,
                position: row.get(10),
            };
            playlist_tracks.push(pt);
        }

        if !playlist_tracks.is_empty() {
            tracks = Some(playlist_tracks);
        }
    }

    let row = &rows[0];
    let playlist = Playlist {
        id: row.get(0),
        name: row.get(1),
        tracks,
    };

    Ok(Box::new(warp::reply::json(&playlist)))
}

pub async fn add_to_playlist(id: i32, t: AddToPlaylist, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let client = db.get().await?;
    let stmt = client.prepare("
        SELECT MAX(position) FROM playlist_track WHERE playlist_id = $1
    ").await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&id]).await.map_err(Error::from)?;

    let next_pos = if rows.is_empty() { 1 } else { rows[0].try_get::<'_, _, i32>(0).unwrap_or(0) + 1 };
    let stmt = client.prepare("
        INSERT INTO playlist_track (playlist_id, track_id, position)
        VALUES ($1, $2, $3)
    ").await.map_err(Error::from)?;
    client.query(&stmt, &[&id, &t.track_id, &next_pos]).await.map_err(Error::from)?;

    Ok(warp::reply::with_status(warp::reply(), StatusCode::CREATED))
}
