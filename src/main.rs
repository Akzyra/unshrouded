use std::fmt::format;
use crate::container::hash::{crc64, fnv1a32};
use crate::container::structs::{KFCFile, ResourceId, ResourceInfo};
use binrw::BinRead;
use std::fs::File;
use std::io::{BufReader, Seek};

mod container;

fn main() {
    let f = File::open("./game/enshrouded.kfc").unwrap();
    let mut r = BufReader::new(f);

    let file = KFCFile::read(&mut r).unwrap();
    println!(
        "header: {:x}, {:?}, {:?}",
        file.resource_start, file.unk1, file.unk2
    );
    println!("comment: {}", String::from_utf8_lossy(&file.comment));
    println!("{:?}", file.one);

    // check read data
    assert_eq!(file.id_list.len(), file.resource_id.len());
    assert_eq!(file.id_list.len(), file.resource_info.len());

    // position is now before resource data
    let pos = r.stream_position().unwrap();
    assert_eq!(pos, file.resource_start.into());
    assert_eq!(pos, file.one[0].resource_start.into());

    let end = r.seek(std::io::SeekFrom::End(0)).unwrap();
    assert_eq!(end - pos, file.one[0].resource_size.into());

    println!("TypeNames: {}", file.type_name_hash.len());
    for i in 0..file.type_name_hash.len() {
        let hash = &file.type_name_hash[i];
        let info = &file.type_name_info[i];
        let bucket = hash & 0b111_1111;
        println!(
            "- {:>8x}  {:>5}  {:>5}  [{bucket}]",
            hash, info.start, info.count
        );
    }

    println!("Content: {}", file.content_hash.len());
    for i in 0..20 {
        let hash = &file.content_hash[i];
        let info = &file.content_info[i];
        let bucket = hash.hash0 & 0x1ffff;
        println!(
            "- {:>8x}  {:02}  {:08x} [{bucket}]",
            hash.hash0, info.dat_file, info.offset,
        );
    }

    println!("Resources: {}", file.resource_id.len());
    for i in 0..20 {
        let id = &file.resource_id[i];
        let id_str = format!("{}", id);
        let id_alt = format!("{}:{:08x}:{}", id.guid, id.type_name, id.part_index);
        let hash = id.hash();

        // what is used to calculate the hash for the hashmap ???
        let fnv_guid = fnv1a32(&id.guid.raw);
        let fnv_guid_hex = fnv1a32(hex::encode(id.guid.raw).as_bytes());
        let fnv_guid_str = fnv1a32(format!("{}", id.guid).as_bytes());

        let fnv_id_str = fnv1a32(id_str.as_bytes());
        let fnv_id_alt = fnv1a32(id_alt.as_bytes());

        let fnv_crc_le = fnv1a32(&hash.to_le_bytes());
        let fnv_crc_be = fnv1a32(&hash.to_be_bytes());
        let fnv_crc_hex = fnv1a32(format!("{hash:016x}").as_bytes());

        println!("- {id_str:50} {hash:016x}   \
        {fnv_guid:>8x} {fnv_guid_hex:>8x} {fnv_guid_str:>8x}   \
        {fnv_id_str:>8x} {fnv_id_alt:>8x}   \
        {fnv_crc_le:>8x} {fnv_crc_be:>8x} {fnv_crc_hex:>8x}");
    }
}
