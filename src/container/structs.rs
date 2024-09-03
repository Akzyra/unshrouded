#![allow(dead_code)]

use binrw::binread;
use std::fmt::{Debug, Display, Formatter};
use std::io::SeekFrom;

use crate::container::hash::crc64;

#[binread]
#[br(little, magic = b"KFC2")]
#[derive(Debug)]
pub struct KFCFile {
    pub resource_start: u32,
    pub unk1: u32,
    pub unk2: u32,

    pub meta: KFCMeta,

    #[br(
      seek_before = SeekFrom::Start(meta.comment.offset ),
      count = meta.comment.count
    )]
    pub comment: Vec<u8>,

    #[br(
      seek_before = SeekFrom::Start(meta.dat_info.offset),
      count = meta.dat_info.count
    )]
    pub dat_info: Vec<KFCDatInfo>,

    // skip unknown zero1
    // skip unknown zero2
    #[br(
      seek_before = SeekFrom::Start(meta.one.offset ),
      count = meta.one.count,
    )]
    pub one: Vec<KFCOne>,

    #[br(
      seek_before = SeekFrom::Start(meta.id_list.offset ),
      count = meta.id_list.count,
    )]
    pub id_list: Vec<u32>,

    #[br(
      seek_before = SeekFrom::Start(meta.content_hashmap.offset ),
      count = meta.content_hashmap.count,
    )]
    pub content_hashmap: Vec<StaticHashMapEntry>,

    #[br(
      seek_before = SeekFrom::Start(meta.content_hash.offset),
      count = meta.content_hash.count,
    )]
    pub content_hash: Vec<ContentHash>,

    #[br(
      seek_before = SeekFrom::Start(meta.content_info.offset ),
      count = meta.content_info.count,
    )]
    pub content_info: Vec<ContentInfo>,

    #[br(
      seek_before = SeekFrom::Start(meta.resource_hashmap.offset ),
      count = meta.resource_hashmap.count,
    )]
    pub resource_hashmap: Vec<StaticHashMapEntry>,

    #[br(
      seek_before = SeekFrom::Start(meta.resource_id.offset ),
      count = meta.resource_id.count,
    )]
    pub resource_id: Vec<ResourceId>,

    #[br(
      seek_before = SeekFrom::Start(meta.resource_info.offset ),
      count = meta.resource_info.count,
    )]
    pub resource_info: Vec<ResourceInfo>,

    #[br(
      seek_before = SeekFrom::Start(meta.type_name_hashmap.offset ),
      count = meta.type_name_hashmap.count,
    )]
    pub type_name_hashmap: Vec<StaticHashMapEntry>,

    #[br(
      seek_before = SeekFrom::Start(meta.type_name_hash.offset ),
      count = meta.type_name_hash.count,
    )]
    pub type_name_hash: Vec<u32>,

    #[br(
      seek_before = SeekFrom::Start(meta.type_name_info.offset),
      count = meta.type_name_info.count,
    )]
    pub type_name_info: Vec<TypeNameInfo>,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct KFCMeta {
    comment: KFCMetaEntry,
    dat_info: KFCMetaEntry,

    zero1: KFCMetaEntry,
    zero2: KFCMetaEntry,

    one: KFCMetaEntry,
    id_list: KFCMetaEntry,

    content_hashmap: KFCMetaEntry,
    content_hash: KFCMetaEntry,
    content_info: KFCMetaEntry,

    resource_hashmap: KFCMetaEntry,
    resource_id: KFCMetaEntry,
    resource_info: KFCMetaEntry,

    type_name_hashmap: KFCMetaEntry,
    type_name_hash: KFCMetaEntry,
    type_name_info: KFCMetaEntry,
}

#[binread]
#[br(little, stream = s)]
#[derive(Debug)]
pub struct KFCMetaEntry {
    pub rel_offset: u32,
    pub count: u32,

    #[br(try_calc = s.stream_position().map(|v: u64| v + rel_offset as u64 - 8))]
    pub offset: u64,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct KFCDatInfo {
    pub size: u64,
    pub count: u64,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct KFCOne {
    pub resource_start: u32,
    pub resource_size: u32,
    pub resource_count: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct StaticHashMapEntry {
    pub start: u32,
    pub count: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct ContentHash {
    pub size: u32,
    pub hash0: u32,
    pub hash1: u32,
    pub hash2: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct ContentInfo {
    pub offset: u32,
    pub pad0: u16,
    pub dat_file: u16,
    pub pad1: u64,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct ResourceId {
    pub guid: Guid,
    pub type_name: u32,
    pub part_index: u32,
    pub reserved0: u32,
    pub reserved1: u32,
}

impl Display for ResourceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{:x}_{}", self.guid, self.type_name, self.part_index)
    }
}

impl ResourceId {
    pub fn hash(&self) -> u64 {
        crc64(format!("{}", self).as_bytes())
    }
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct ResourceInfo {
    pub rel_offset: u32,
    pub size: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct TypeNameInfo {
    pub hash2: u32,
    pub start: u32,
    pub count: u32,
}

#[binread]
#[br(little)]
pub struct Guid {
    #[br(restore_position)]
    pub raw: [u8; 16],

    pub time_low: u32,
    pub time_mid: u16,
    pub time_hi_and_version: u16,
    pub clock_seq_high_and_reserved: u8,
    pub clock_seq_low: u8,
    pub node: [u8; 6],
}

impl Debug for Guid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self, hex::encode(self.raw))
    }
}

impl Display for Guid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{}",
            self.time_low,
            self.time_mid,
            self.time_hi_and_version,
            self.clock_seq_high_and_reserved,
            self.clock_seq_low,
            hex::encode(self.node),
        )
    }
}
