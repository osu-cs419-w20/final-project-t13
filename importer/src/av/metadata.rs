use std::collections::HashMap;
use std::cell::Cell;
use std::path::Path;

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
            Album => md.iter().find(|&(k, _)| k == &"album" || k == &"ALBUM").map(|(_, a)| MetadataValue::Album(a)),
            Artist => md.iter().find(|&(k, _)| k == &"artist" || k == &"ARTIST").map(|(_, a)| MetadataValue::Artist(a)),
            Disc => md
                .iter()
                .find(|&(k, _)| k == &"disc" || k == &"DISC")
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
                .find(|&(k, _)| k == &"track" || k == &"TRACKNUMBER")
                .and_then(|(_, t)| u16::from_str_radix(t, 10).ok())
                .map(|t| MetadataValue::TrackNumber(t)),
            TrackCount => md
                .iter()
                .find(|&(k, _)| k == &"TRACKTOTAL" || k == &"TOTALTRACKS")
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
        Ok(Track {
            ctx,
            metadata,
            format,
        })
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
