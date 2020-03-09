use std::collections::BTreeSet;

use crate::Error;
use crate::db::DB;
use crate::filters::{RelationsOption, PaginationOptions};

#[derive(Serialize)]
pub struct Artist {
    pub id: i32,
    pub mbid: String,
    pub name: String,
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums: Option<Vec<super::albums::Album>>,
}

// GET /artists(?page=X&limit=Y)
pub async fn get_artists(opts: PaginationOptions, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let client = db.get().await?;
    let count = db.count_rows("artist", &client).await.map_err(Error::from)?.unwrap_or(0);
    let (limit, page) = match (opts.limit.or(Some(15)), opts.page) {
        (Some(l), Some(p)) => {
            if count < p * l {
                (l, (count as f64 / l as f64).ceil() as i64)
            } else {
                (l, p)
            }
        }
        (Some(l), None) => (l, 1),
        _ => (15, 1),
    };
    let offset = (page - 1) * limit as i64;
    let total_pages = (count as f64 / limit as f64).ceil() as i64;
    let stmt = client.prepare("
        SELECT A.id, A.mbid, A.name, I.url as image_url
        FROM artist A
        INNER JOIN artist_image I
            ON I.artist_id = A.id
        LIMIT $1 OFFSET $2
    ").await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&limit, &offset]).await.map_err(Error::from)?;
    let mut artists = Vec::new();

    for row in rows {
        let artist = Artist {
            id: row.get(0),
            mbid: row.get(1),
            name: row.get(2),
            image_url: row.get(3),
            albums: None,
        };
        artists.push(artist);
    }

    let res = super::PaginatedResponse {
        page,
        count,
        total_pages,
        data: artists,
    };

    Ok(warp::reply::json(&res))
}

// GET /artists/:id
pub async fn get_artist_with_id(id: i32, rels: RelationsOption, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let client = db.get().await?;
    let rels = rels.relations.unwrap_or(BTreeSet::new());
    let mut select_fields = vec!["A.id", "A.mbid", "A.name", "I.url as image_url"];
    let mut joins = vec!["INNER JOIN artist_image I ON I.artist_id = A.id"];

    let mut loading_albums = false;

    for rel in rels {
        use crate::filters::Relation;
        match rel {
            Relation::Albums => {
                select_fields.extend_from_slice(&[
                    "R.id",
                    "R.mbid",
                    "R.title",
                    "R.artist_id",
                    "RI.url as album_image_url"
                ]);
                joins.extend_from_slice(&[
                    "LEFT OUTER JOIN album R ON R.artist_id = A.id",
                    "LEFT OUTER JOIN album_image RI ON RI.album_id = R.id",
                ]);
                loading_albums = true;
            }
            _ => {}
        }
    }

    let select_fields = select_fields.as_slice().join(", ");
    let joins = joins.as_slice().join("\n");
    let q = format!("SELECT {} FROM artist A {} WHERE A.id = $1", select_fields, joins);
    let stmt = client.prepare(&q).await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&id]).await.map_err(Error::from)?;

    if rows.len() < 1 {
        return Err(warp::reject());
    }

    let mut albums = None;

    if loading_albums {
        let mut albums_vec = Vec::new();
        for row in &rows {
            // If there's no ID, no album record exists on this row, so
            // skip it.
            if let Err(_) = row.try_get::<'_, _, i32>(4) {
                continue;
            }
            let album = crate::handlers::albums::Album {
                id: row.get(4),
                mbid: row.get(5),
                title: row.get(6),
                artist_id: row.get(7),
                image_url: row.try_get(8).ok(),
                artist: None,
                tracks: None,
            };
            albums_vec.push(album);
        }

        if !albums_vec.is_empty(){
            albums = Some(albums_vec);
        }
    }

    let row = &rows[0];
    let artist = Artist {
        id: row.get(0),
        mbid: row.get(1),
        name: row.get(2),
        image_url: row.get(3),
        albums,
    };

    Ok(warp::reply::json(&artist))
}
