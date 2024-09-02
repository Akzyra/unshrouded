use crate::container::structs::KFCFile;
use binrw::BinRead;
use std::fs::File;
use std::io::{BufReader, Seek};

mod container;

fn main() {
    let f = File::open("C:\\Steam\\steamapps\\common\\Enshrouded\\enshrouded.kfc").unwrap();
    let mut r = BufReader::new(f);

    let file = KFCFile::read(&mut r).unwrap();
    println!(
        "header: {:x}, {:?}, {:?}",
        file.resource_start, file.unk1, file.unk2
    );
    println!("comment: {}", String::from_utf8_lossy(&file.comment));
    for di in file.dat_info {
        println!("- {:?}", di);
    }
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
}
