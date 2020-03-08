use std::pin::Pin;

use reqwest::header::{self, HeaderMap};
use serde::Deserialize;

use super::entities::{Recording, Relation};

const API_BASE_URL: &'static str = "http://musicbrainz.org/ws/2";
const DOPLR_VERSION: &'static str = env!("CARGO_PKG_VERSION");

type Result<T> = std::result::Result<T, super::Error>;

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
}

#[derive(Debug, Deserialize)]
pub enum SearchResult {
    #[serde(rename = "recordings")]
    Recordings(Vec<Recording>),
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub created: String,
    pub count: usize,
    pub offset: usize,
    #[serde(flatten)]
    pub results: SearchResult,
}

#[derive(Debug, Deserialize)]
pub struct ArtistResponse {
    name: String,
    #[serde(rename = "sort-name")]
    sort_name: String,
    relations: Vec<Relation>,
}

impl Client {
    pub fn new() -> Result<Self> {
        let http = reqwest::ClientBuilder::new()
            .default_headers(Self::default_headers())
            .build()?;

        Ok(Client {
            http
        })
    }

    pub async fn get_artist(&self, id: &str) -> Result<ArtistResponse> {
        let url = format!("{}/artist/{}", API_BASE_URL, id);
        let res = self.http.get(&url)
            .query(&[("inc", "url-rels")])
            .send()
            .await?;
        let buf = res.bytes().await?;
        let res: ArtistResponse = serde_json::from_reader(buf.as_ref()).unwrap();
        Ok(res)
    }

    pub async fn search_recordings(&self, track: &crate::av::metadata::Track<'_>) -> Result<SearchResponse> {
        let q = build_query_from_track(track);
        let url = API_BASE_URL.to_string() + "/recording";
        let res = self.http.get(&url)
            .query(&[("query", q)])
            .send()
            .await?;
        let buf = res.bytes().await?;
        let res: SearchResponse = serde_json::from_reader(buf.as_ref()).unwrap();
        Ok(res)
    }

    fn default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        // MusicBrainz rejects requests without valid user agent
        let ua = format!("Doplr/{} ( doplr@jcndrop.com )", DOPLR_VERSION);
        headers.insert(header::USER_AGENT, ua.parse().unwrap());
        // Use the JSON API rather than the default XML API
        headers.insert(header::ACCEPT, "application/json".parse().unwrap());
        headers
    }
}

fn build_query_from_track(track: &crate::av::metadata::Track<'_>) -> String {
    let md = track.metadata();

    let mut fields = Vec::new();
    fields.push(md.album.as_ref());
    fields.push(md.artist.as_ref());
    fields.push(md.track_title.as_ref());
    fields.push(md.track_number.as_ref());
    fields.push(md.track_count.as_ref());
    fields.push(md.track_length.as_ref());

    fields
        .into_iter()
        .filter(|m| m.is_some())
        .filter_map(|m| {
            use crate::av::metadata::MetadataValue::*;
            match m {
                Some(Album(a)) => Some(format!("release:{}", a)),
                Some(Artist(a)) => Some(format!("(artist:{0} OR artistname:{0} OR creditname:{0})", a)),
                Some(TrackTitle(t)) => Some(format!("(recording:{0} OR recordingaccent:{0})", t)),
                Some(TrackNumber(n)) => Some(format!("tnum:{}", n)),
                Some(TrackCount(c)) => Some(format!("tracks:{}", c)),
                Some(TrackLength(l)) => Some(format!("dur:{}", (*l as u64) * 1000)),
                _ => None,
            }
        })
        .collect::<Vec<String>>()
        .join(" AND ")
}
