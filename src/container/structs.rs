#![allow(dead_code)]

use binrw::binread;
use std::io::SeekFrom;

#[binread]
#[br(little, magic = b"KFC2")]
#[derive(Debug)]
pub struct KFCFile {
    pub resource_start: u32,
    pub unk1: u32,
    pub unk2: u32,

    #[br(temp)]
    _comment: ArrayInfo,
    #[br(temp)]
    _dat_info: ArrayInfo,

    #[br(temp)]
    _zero1: ArrayInfo,
    #[br(temp)]
    _zero2: ArrayInfo,

    #[br(temp)]
    _one: ArrayInfo,
    #[br(temp)]
    _id_list: ArrayInfo,

    #[br(temp)]
    content_hashmap: ArrayInfo,
    #[br(temp)]
    content_hash: ArrayInfo,
    #[br(temp)]
    content_info: ArrayInfo,

    #[br(temp)]
    _resource_hashmap: ArrayInfo,
    #[br(temp)]
    _resource_id: ArrayInfo,
    #[br(temp)]
    _resource_info: ArrayInfo,

    #[br(temp)]
    _type_name_hashmap: ArrayInfo,
    #[br(temp)]
    _type_name_hash: ArrayInfo,
    #[br(temp)]
    _type_name_info: ArrayInfo,

    #[br(
      seek_before = SeekFrom::Start(_comment.offset ),
      count = _comment.count,
    )]
    pub comment: Vec<u8>,

    #[br(
      seek_before = SeekFrom::Start(_dat_info.offset),
      count = _dat_info.count,
    )]
    pub dat_info: Vec<KFCDatInfo>,

    // skip unknown zero1
    // skip unknown zero2
    #[br(
      seek_before = SeekFrom::Start(_one.offset ),
      count = _one.count,
    )]
    pub one: Vec<KFCOne>,

    #[br(
      seek_before = SeekFrom::Start(_id_list.offset ),
      count = _id_list.count,
    )]
    pub id_list: Vec<u32>,

    #[br(
      seek_before = SeekFrom::Start(content_hashmap.offset ),
      count = content_hashmap.count,
    )]
    pub content_hashmap: Vec<StaticHashMapEntry>,

    #[br(
      seek_before = SeekFrom::Start(content_hash.offset),
      count = content_hash.count,
    )]
    pub content_hash: Vec<ContentHash>,

    #[br(
      seek_before = SeekFrom::Start(content_info.offset ),
      count = content_info.count,
    )]
    pub content_info: Vec<ContentInfo>,

    #[br(
      seek_before = SeekFrom::Start(_resource_hashmap.offset ),
      count = _resource_hashmap.count,
    )]
    pub resource_hashmap: Vec<StaticHashMapEntry>,

    #[br(
      seek_before = SeekFrom::Start(_resource_id.offset ),
      count = _resource_id.count,
    )]
    pub resource_id: Vec<ResourceId>,

    #[br(
      seek_before = SeekFrom::Start(_resource_info.offset ),
      count = _resource_info.count,

    )]
    pub resource_info: Vec<ResourceInfo>,

    #[br(
      seek_before = SeekFrom::Start(_type_name_hashmap.offset ),
      count = _type_name_hashmap.count,
    )]
    pub type_name_hashmap: Vec<StaticHashMapEntry>,

    #[br(
      seek_before = SeekFrom::Start(_type_name_hash.offset ),
      count = _type_name_hash.count,
    )]
    pub type_name_hash: Vec<u32>,

    #[br(
      seek_before = SeekFrom::Start(_type_name_info.offset),
      count = _type_name_info.count,
    )]
    pub type_name_info: Vec<TypeNameInfo>,
}

#[binread]
#[br(little, stream = s)]
#[derive(Debug)]
pub struct ArrayInfo {
    pub _rel_offset: u32,
    pub count: u32,

    #[br(try_calc = s.stream_position().map(|v: u64| v + _rel_offset as u64 - 8))]
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
    pub guid: u128,
    pub type_name: u32,
    pub part_index: u32,
    pub reserved0: u32,
    pub reserved1: u32,
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
