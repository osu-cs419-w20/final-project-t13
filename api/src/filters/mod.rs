use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor};
use warp::Filter;

pub mod albums;
pub mod artists;
pub mod playlists;
pub mod tracks;

pub fn build(db: crate::db::DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    albums::albums_filters(db.clone())
        .or(artists::artists_filters(db.clone()))
        .or(tracks::tracks_filters(db.clone()))
        .or(playlists::playlists_filters(db))
}

fn db_filter(db: crate::db::DB)
    -> impl Filter<Extract = (crate::db::DB, ), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || db.clone())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Relation {
    Albums,
    Artist,
    Tracks,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RelationsOption {
    #[serde(deserialize_with = "deserialize_relations")]
    pub relations: Option<BTreeSet<Relation>>,
}

fn deserialize_relations<'de, D>(deserializer: D) -> Result<Option<BTreeSet<Relation>>, D::Error>
    where
        D: Deserializer<'de>,
{
    struct RelationsVisitor;

    impl<'de> Visitor<'de> for RelationsVisitor {
        type Value = Option<BTreeSet<Relation>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("any combination of `artist`, `album`, and `track`")
        }

        fn visit_str<E>(self, value: &str) -> Result<Option<BTreeSet<Relation>>, E>
        where
            E: de::Error,
        {
            let mut set = BTreeSet::new();
            for rel in value.split(",") {
                match rel.trim() {
                    "albums" => { set.insert(Relation::Albums); }
                    "artist" => { set.insert(Relation::Artist); }
                    "tracks" => { set.insert(Relation::Tracks); }
                    _ => {},
                }
            }
            Ok(Some(set))
        }
    }

    deserializer.deserialize_str(RelationsVisitor)
}

#[derive(Debug, Deserialize)]
pub struct PaginationOptions {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

