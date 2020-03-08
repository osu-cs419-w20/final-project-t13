use crate::av::metadata::{MetadataValue, Track};
use crate::metadata::providers::MBClient;
use crate::metadata::providers::musicbrainz::SearchResult;
use crate::metadata::providers::musicbrainz::entities;
use crate::utils::lev::damlev;

pub struct TrackImporter<'a> {
    client: MBClient,
    track: Track<'a>,
}

impl<'a> TrackImporter<'a> {
    pub fn new(client: MBClient, track: Track<'a>) -> TrackImporter<'a> {
        TrackImporter {
            client,
            track,
        }
    }

    pub async fn import(&self) {
        let rec = self.match_to_recording().await.unwrap();
        let release = match rec.releases.as_ref() {
            Some(releases) => self.match_release(&releases),
            _ => None,
        }.unwrap();
        let artist_credit = release.artist_credit.as_ref().unwrap().first().unwrap();
        let artist = self.client.get_artist(&artist_credit.artist.id).await.unwrap();
        println!("Release :{:#?}", release);
        println!("Recording:{:#?}", rec);
        println!("Artist:{:#?}", artist);
    }

    pub async fn search_recordings(&self) -> Vec<entities::Recording> {
        let res = self.client.search_recordings(&self.track).await.unwrap();
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

                // For now, heavily prefer CD releases
                let not_cd_penalty = !r.media.iter().any(|m| m.format == Some("CD".to_string()));
                if not_cd_penalty { score += 10; }

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
