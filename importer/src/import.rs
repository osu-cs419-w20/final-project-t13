use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use regex::Regex;

use crate::av::metadata::{MetadataValue, Track as AVTrack};
use crate::metadata::providers::{MBClient, SpotifyClient};
use crate::metadata::providers::musicbrainz::SearchResult;
use crate::metadata::providers::musicbrainz::entities;
use crate::utils::lev::damlev;

use crate::models::*;

pub struct TrackImporter<'a> {
    mb_client: MBClient,
    spotify_client: SpotifyClient,
    track: AVTrack<'a>,
}

impl<'a> TrackImporter<'a> {
    pub fn new(mb_client: MBClient, spotify_client: SpotifyClient, track: AVTrack<'a>) -> TrackImporter<'a> {
        TrackImporter {
            mb_client,
            spotify_client,
            track,
        }
    }

    pub async fn find_match(&self) -> (entities::ArtistCredit, entities::Release, entities::Recording) {
        let rec = self.match_to_recording().await.unwrap();
        let release = match rec.releases.as_ref() {
            Some(releases) => self.match_release(&releases),
            _ => None,
        }.unwrap();
        let artist_credit = release.artist_credit.as_ref().unwrap().first().unwrap().clone();
        (artist_credit, release, rec)
    }

    pub fn build_track(&self, rec: &entities::Recording, release: &entities::Release) -> Track {
        let position = release.media.first().as_ref()
            .map(|m| {
                m.track.first().as_ref().map(|t| u16::from_str_radix(&t.number, 10).unwrap()).unwrap()
            })
            .unwrap();
        let file_location = self.track.path_str().unwrap().to_string();
        let file_location = file_location.as_str().trim_start_matches("/Users/jason/j/tmp/dtst");
        const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

        Track {
            mbid: rec.id.clone().unwrap(),
            title: rec.title.clone().unwrap(),
            position,
            bitrate: self.track.bit_rate(),
            duration: self.track.duration(),
            file_location: utf8_percent_encode(&format!("/static{}", file_location), FRAGMENT).to_string(),
        }
    }

    pub async fn build_artist(&self, artist_credit: &entities::ArtistCredit) -> Artist {
        let artist = self.mb_client.get_artist(&artist_credit.artist.id).await.unwrap();
        let spotify_artist = match find_artist_spotify_id(&artist.relations) {
            Some(id) => Some(self.spotify_client.get_artist(&id).await.unwrap()),
            None => None,
        };
        let artist_image = spotify_artist.and_then(|a| {
            a.images.iter().max_by_key(|i| i.width.unwrap_or(0)).map(|i| i.url.clone())
        });

        Artist {
            mbid: artist_credit.artist.id.clone(),
            name: artist_credit.artist.name.clone(),
            image_url: artist_image,
        }
    }

    pub async fn build_album(&self, release: &entities::Release, artist_credit: &entities::ArtistCredit) -> Album {
        // Check if Covert Art Archive has this release
        let image_url = match self.mb_client.get_cover_art(&release.id).await.unwrap() {
            Some(cover_art) => {
                cover_art.images
                    .iter()
                    .filter(|i| i.front)
                    .next()
                    .map(|i| i.image.clone())
            }
            // If release is not on Covert Art Archive, try the Spotify API
            None => {
                match self.spotify_client.search_album(&release.title, &artist_credit.artist.name).await {
                    Ok(albums) => {
                        albums
                            .into_iter()
                            .map(|a| (damlev(&a.name, &release.title), a))
                            .min_by_key(|(s, _)| *s)
                            .and_then(|(_, a)| {
                                a.images
                                    .into_iter()
                                    .max_by_key(|i| i.width.unwrap_or(0))
                                    .map(|i| i.url)
                            })
                    }
                    _ => None
                }
            }
        };

        Album {
            mbid: release.id.clone(),
            name: release.title.clone(),
            image_url,
        }
    }

    pub async fn import(&self) -> (Artist, Album, Track) {
        let (artist_credit, release, rec) = self.find_match().await;
        self.from_entities(&rec, &release, &artist_credit).await
    }

    pub async fn from_entities(&self, rec: &entities::Recording, release: &entities::Release, artist_credit: &entities::ArtistCredit) -> (Artist, Album, Track) {
        let artist = self.build_artist(artist_credit).await;
        let album = self.build_album(release, artist_credit).await;
        let track = self.build_track(rec, release);
        (artist, album, track)
    }

    pub async fn search_recordings(&self) -> Vec<entities::Recording> {
        let res = self.mb_client.search_recordings(&self.track).await.unwrap();
        match res.results {
            SearchResult::Recordings(r) => r,
            _ => panic!("unexpected deserialization result"),
        }
    }

    pub fn match_release(&self, releases: &Vec<entities::Release>) -> Option<entities::Release> {
        if releases.is_empty() { return None; }
        else if releases.len() == 1 { return Some(releases[0].clone()); }

        let md = self.track.metadata();
        let on_cd = self.track.guess_is_cd();
        releases.iter()
            .map(|r| {
                let mut score = 0;
                match md.album {
                    Some(MetadataValue::Album(a)) => score += damlev(&r.title, a),
                    _ => {}
                }

                match r.artist_credit.as_ref() {
                    Some(ac) => {
                        score += ac.iter()
                            .map(|a| {
                                match (a.name.as_ref(), md.artist.as_ref()) {
                                    (Some(a1), Some(MetadataValue::Artist(a2))) => damlev(&a1, &a2),
                                    _ => 0,
                                }
                            })
                            .min()
                            .unwrap_or(0);
                    }
                    None => {}
                }

                match md.track_count {
                    Some(MetadataValue::TrackCount(c)) => {
                        let count_match = r.media.iter().any(|m| match m.track_count {
                            Some(mc) => c == mc,
                            None => false,
                        });
                        if !count_match { score += 5 };
                    }
                    _ => {}
                }

                match md.track_number {
                    Some(MetadataValue::TrackNumber(n)) => {
                        let num_match = r.media.iter().any(|m| match m.track_offset {
                            Some(o) => (o + 1) == n,
                            None => false,
                        });
                        if !num_match { score += 5 };
                    }
                    _ => {}
                }

                if r.country != Some("US".to_string()) {
                    score += 1;
                }
                (score, r)
            })
            .min_by_key(|(score, _) | *score)
            .map(|(_, r)| r.clone())
    }

    pub async fn match_to_recording(&self) -> Option<entities::Recording> {
        let mut recs = self.search_recordings().await;

        if recs.is_empty() { return None; }
        else if recs.len() == 1 { return recs.pop(); }

        let md = self.track.metadata();
        recs.into_iter()
            .map(|r| {
                let mut score = 0;
                match (r.title.as_ref(), md.track_title.as_ref()) {
                    (Some(t1), Some(MetadataValue::TrackTitle(t2))) => score += damlev(&t1, &t2),
                    _ => {}
                }

                if md.artist.is_some() && r.artist_credit.len() > 0 {
                    score += r.artist_credit.iter()
                        .map(|a| {
                            match (a.name.as_ref(), md.artist.as_ref()) {
                                (Some(a1), Some(MetadataValue::Artist(a2))) => damlev(&a1, &a2),
                                _ => 0,
                            }
                        })
                        .min()
                        .unwrap_or(0);
                }

                match r.releases.as_ref() {
                    Some(rs) => {
                        let on_cd = self.track.guess_is_cd();
                        let r_score = rs.iter()
                            .filter(|r| {
                            if !on_cd { true }
                            else {
                                r.media
                                    .iter()
                                    .map(|m| m.format == Some("CD".to_string()))
                                    .any(|b| b)
                            }
                        })
                        .map(|r| {
                            let mut score = 0;
                            match md.album {
                                Some(MetadataValue::Album(t2)) => score += damlev(&r.title, t2),
                                _ => {}
                            }
                            score
                        })
                        .min()
                        .unwrap_or(0);
                    score += r_score;
                }
                None => {}
            }
            (score, r)
        })
        .min_by_key(|(score, _)| *score)
        .map(|(_, r)| r)
    }
}

fn find_artist_spotify_id(relations: &Vec<entities::Relation>) -> Option<String> {
    let reg = Regex::new(r"^https://open\.spotify\.com/artist/([a-zA-Z0-9-_]+)$").unwrap();
    for r in relations {
        match &r.url {
            Some(r) => {
                let caps = reg.captures(&r.resource);
                match caps {
                    Some(caps) => {
                        return Some(caps.get(1).unwrap().as_str().to_string());
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
    None
}
