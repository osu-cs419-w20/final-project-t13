use std::collections::HashMap;
use std::convert::TryInto;
use std::path::Path;

use regex::Regex;
use walkdir::WalkDir;

use super::format::AVFormatContext;

pub enum MetadataKey {
    Artist,
    Album,
    Disc,
    DiscCount,
    TrackTitle,
    TrackNumber,
    TrackCount,
    TrackLength,
}

#[derive(Debug)]
pub enum MetadataValue<'a> {
    Artist(&'a str),
    Album(&'a str),
    Disc(u8),
    DiscCount(u8),
    TrackTitle(&'a str),
    TrackNumber(u16),
    TrackCount(u16),
    TrackLength(u32),
}

pub struct FLAC;
pub struct MP3;

pub trait TrackFormat {
    fn try_get_metadata<'a, 'b>(&self, md: &'a HashMap<&'b str, &'b str>, item: MetadataKey) -> Option<MetadataValue<'b>>;
}

impl TrackFormat for FLAC {
    fn try_get_metadata<'a, 'b>(&self, md: &'a HashMap<&'b str, &'b str>, item: MetadataKey) -> Option<MetadataValue<'b>> {
        use MetadataKey::*;
        match item {
            Album => md.iter().find(|&(k, _)| k.to_lowercase() == "album").map(|(_, a)| MetadataValue::Album(a)),
            Artist => md.iter().find(|&(k, _)| k.to_lowercase() == "artist").map(|(_, a)| MetadataValue::Artist(a)),
            Disc => md
                .iter()
                .find(|&(k, _)| k.to_lowercase() == "disc")
                .and_then(|(_, d)| u8::from_str_radix(d, 10).ok())
                .map(|d| MetadataValue::Disc(d)),
            DiscCount => md
                .iter()
                .find(|&(k, _)| k.to_lowercase() == "disctotal" || k.to_lowercase() == "totaldiscs")
                .and_then(|(_, d)| u8::from_str_radix(d, 10).ok())
                .map(|d| MetadataValue::DiscCount(d)),
            TrackTitle => md.iter().find(|&(k, _)| k.to_lowercase() == "title").map(|(_, t)| MetadataValue::TrackTitle(t)),
            TrackNumber => md
                .iter()
                .find(|&(k, _)| k.to_lowercase() == "track" || k.to_lowercase() == "tracknumber")
                .and_then(|(_, t)| u16::from_str_radix(t, 10).ok())
                .map(|t| MetadataValue::TrackNumber(t)),
            TrackCount => md
                .iter()
                .find(|&(k, _)| k.to_lowercase() == "tracktotal" || k.to_lowercase() == "totaltracks")
                .and_then(|(_, t)| u16::from_str_radix(t, 10).ok())
                .map(|t| MetadataValue::TrackCount(t)),
            _ => None
        }
    }
}

impl TrackFormat for MP3 {
    fn try_get_metadata<'a, 'b>(&self, md: &'a HashMap<&'b str, &'b str>, item: MetadataKey) -> Option<MetadataValue<'b>> {
        use MetadataKey::*;
        match item {
            Album => md.iter().find(|&(k, _)| k == &"album" || k == &"ALBUM").map(|(_, a)| MetadataValue::Album(a)),
            Artist => md.iter().find(|&(k, _)| k == &"artist" || k == &"ARTIST").map(|(_, a)| MetadataValue::Artist(a)),
            Disc => md
                .iter()
                .find(|&(k, _)| k == &"DISCNUMBER")
                .and_then(|(_, d)| u8::from_str_radix(d, 10).ok())
                .map(|d| MetadataValue::Disc(d)),
            DiscCount => md
                .iter()
                .find(|&(k, _)| k == &"DISCTOTAL")
                .and_then(|(_, d)| u8::from_str_radix(d, 10).ok())
                .map(|d| MetadataValue::DiscCount(d)),
            TrackTitle => md.iter().find(|&(k, _)| k == &"title" || k == &"TITLE").map(|(_, t)| MetadataValue::TrackTitle(t)),
            TrackNumber => md
                .iter()
                .find(|&(k, _)| k == &"TRACK")
                .and_then(|(_, t)| u16::from_str_radix(t, 10).ok())
                .map(|t| MetadataValue::TrackNumber(t)),
            TrackCount => md
                .iter()
                .find(|&(k, _)| k == &"TRACKTOTAL" || k == &"TOTALTRACKS")
                .and_then(|(_, t)| u16::from_str_radix(t, 10).ok())
                .map(|t| MetadataValue::TrackNumber(t)),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum MediaFormat {
    CD,
    Digital,
}

pub struct Track<'a> {
    ctx: AVFormatContext,
    metadata: TrackMetadata<'a>,
    format: Box<dyn TrackFormat>,
}

impl<'a> Track<'a> {
    pub fn from_ctx(ctx: AVFormatContext) -> super::Result<Track<'a>> {
        let format: Box<dyn TrackFormat> = match ctx.determine_format()? {
            super::format::Format::FLAC => Box::new(FLAC),
            super::format::Format::MP3 => Box::new(MP3),
        };
        let raw_metadata = ctx.metadata()?;
        let metadata = TrackMetadata::from_raw_metadata(&raw_metadata, format.as_ref());

        let mut track = Track {
            ctx,
            metadata,
            format,
        };

        if track.metadata.track_count.is_none() {
            track.metadata.track_count = track.guess_track_count().map(|c| MetadataValue::TrackCount(c));
        }

        Ok(track)
    }

    pub fn new<P: AsRef<Path>>(p: P) -> super::Result<Track<'a>> {
        let ctx = AVFormatContext::open(p)?;
        Self::from_ctx(ctx)
    }

    pub fn metadata(&self) -> &TrackMetadata<'a> {
        &self.metadata
    }

    pub fn guess_is_cd(&self) -> bool {
        let md = self.metadata();
        let has_disc_metadata = md.disc.is_some() || md.disc_count.is_some();
        has_disc_metadata
    }

    pub fn guess_media_format(&self) -> Option<MediaFormat> {
        let fmt = self.ctx.path().parent().and_then(|p| p.to_str()).and_then(|path| {
            let reg = Regex::new(r"^.+?(?:\[(CD|WEB)(?:[\s0-9A-Z- ]*?)\]|\((CD|WEB)(?:[\s0-9A-Z- ]*?)\)).*$").unwrap();
            match reg.captures(path) {
                Some(caps) => {
                    let idx = if caps.get(1).is_some() { 1 } else { 2 };
                    match caps.get(idx).unwrap().as_str() {
                        "CD" => Some(MediaFormat::CD),
                        "WEB" => Some(MediaFormat::Digital),
                        _ => None,
                    }
                }
                _ => None,
            }
        });

        if fmt.is_some() {
            fmt
        } else if self.guess_is_cd() {
            Some(MediaFormat::CD)
        } else {
            None
        }
    }

    pub fn bit_rate(&self) -> i64 {
        self.ctx.bit_rate()
    }

    pub fn duration(&self) -> i64 {
        self.ctx.duration()
    }

    pub fn path_str(&self) -> Option<&str> {
        self.ctx.path().to_str()
    }

    pub fn guess_track_count(&self) -> Option<u16> {
        match self.ctx.path().parent() {
            Some(path) => {
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|e| !e.file_type().is_dir())
                    .filter(|e| match e.path().extension() {
                        Some(ext) => {
                            match ext.to_str() {
                                Some("flac") | Some("mp3") => true,
                                _ => false,
                            }
                        }
                        None => false,
                    })
                    .count()
                    .try_into()
                    .ok()
            }
            None => None
        }
    }
}

#[derive(Debug)]
pub struct TrackMetadata<'a> {
    pub album: Option<MetadataValue<'a>>,
    pub artist: Option<MetadataValue<'a>>,
    pub disc: Option<MetadataValue<'a>>,
    pub disc_count: Option<MetadataValue<'a>>,
    pub track_title: Option<MetadataValue<'a>>,
    pub track_number: Option<MetadataValue<'a>>,
    pub track_count: Option<MetadataValue<'a>>,
    pub track_length: Option<MetadataValue<'a>>,
}

impl<'a> TrackMetadata<'a> {
    fn from_raw_metadata(md: &HashMap<&'a str, &'a str>, f: &dyn TrackFormat) -> TrackMetadata<'a> {
        TrackMetadata {
            album: f.try_get_metadata(md, MetadataKey::Album),
            artist: f.try_get_metadata(md, MetadataKey::Artist),
            disc: f.try_get_metadata(md, MetadataKey::Disc),
            disc_count: f.try_get_metadata(md, MetadataKey::DiscCount),
            track_title: f.try_get_metadata(md, MetadataKey::TrackTitle),
            track_number: f.try_get_metadata(md, MetadataKey::TrackNumber),
            track_count: f.try_get_metadata(md, MetadataKey::TrackCount),
            track_length: f.try_get_metadata(md, MetadataKey::TrackLength),
        }
    }
}
