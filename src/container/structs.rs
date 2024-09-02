pub const KFC_MAGIC: &[u8; 4] = b"KFC2";

pub struct KFCFile {
    pub header: KFCHeader,
    pub meta: KFCMeta,

    pub comment: String,
    pub dat_info: Vec<KFCDatInfo>,

    // skip zero1, zero for now
    pub one: Vec<KFCOne>,
    pub id_list: Vec<u32>,

    pub content_hashmap: Vec<StaticHashMap>,
    pub content_hash: Vec<ContentHash>,
    pub content_info: Vec<ContentInfo>,

    pub resource_hashmap: Vec<StaticHashMap>,
    pub resource_id: Vec<ResourceId>,
    pub resource_info: Vec<ResourceInfo>,

    pub type_name_hashmap: Vec<StaticHashMap>,
    pub type_name_hash: Vec<u32>,
    pub type_name_info: Vec<TypeNameInfo>,
}

pub struct KFCHeader {
    pub magic: [u8; 4],
    pub resource_start: u32,
    pub unk1: u32,
    pub unk2: u32,
}

pub struct KFCMetaEntry {
    pub rel_offset: u32,
    pub count: u32,
}
pub struct KFCMeta {
    pub comment: KFCMetaEntry,
    pub dat_info: KFCMetaEntry,

    pub zero1: KFCMetaEntry,
    pub zero2: KFCMetaEntry,

    pub one: KFCMetaEntry,
    pub id_list: KFCMetaEntry,

    pub content_hashmap: KFCMetaEntry,
    pub content_hash: KFCMetaEntry,
    pub content_info: KFCMetaEntry,

    pub resource_hashmap: KFCMetaEntry,
    pub resource_id: KFCMetaEntry,
    pub resource_info: KFCMetaEntry,

    pub type_name_hashmap: KFCMetaEntry,
    pub type_name_hash: KFCMetaEntry,
    pub type_name_info: KFCMetaEntry,
}

pub struct KFCDatInfo {
    pub size: u64,
    pub count: u64,
}

pub struct KFCOne {
    pub resource_start: u32,
    pub resource_size: u32,
    pub resource_count: u32,
}

pub struct StaticHashMap {
    pub start: u32,
    pub count: u32,
}

pub struct ContentHash {
    pub size: u32,
    pub hash0: u32,
    pub hash1: u32,
    pub hash2: u32,
}

pub struct ContentInfo {
    pub offset: u32,
    pub pad0: u16,
    pub dat_file: u16,
    pub pad1: u64,
}

pub struct ResourceId {
    pub guid: u128,
    pub type_name: u32,
    pub part_index: u32,
    pub reserved0: u32,
    pub reserved1: u32,
}
pub struct ResourceInfo {
    pub rel_offset: u32,
    pub size: u32,
}

pub struct TypeNameInfo {
    pub hash2: u32,
    pub start: u32,
    pub count: u32,
}
