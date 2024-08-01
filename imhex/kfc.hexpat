#pragma description Enshouded KFC file
#pragma author Akzyra

// if using real length for the large arrays, increase limits:
//#pragma pattern_limit 51000
//#pragma pattern_limit 550000

import std.string;
import std.core;

struct Header {
    char magic[4]; 
    u32 metaEnd;
    u32 unk1;
    u32 unk2;
};

struct MetaEntry {
    u32 relOffset;
    u32 length;

    u32 offset = addressof(relOffset) + relOffset;
}[[ comment(std::format("0x{:08x}, {:10}",offset, length)) ]];

struct Meta {
    MetaEntry comment;
    MetaEntry datInfo;
    MetaEntry zero1;

    MetaEntry zero2;
    MetaEntry one;
    MetaEntry idList;
    
    MetaEntry block1_weird; // length 2**17
    MetaEntry block1_list1;
    MetaEntry block1_list2;
    
    MetaEntry block2_weird; // length 2**16
    MetaEntry block2_list1; // same length as idList
    MetaEntry chunkInfo;    // same length as idList
    
    MetaEntry block3_weird; // length 2**7
    MetaEntry block3_list1;
    MetaEntry block3_list2;
};

struct DatInfo {
    u64 size;
    u64 count;
};

struct One {
    u32 dataStart;
    u32 dataSize;
    u32 count; // same length as idList
};

struct Weird {
    u32 value;
    u32 addNext;
};

struct Block1_Entry1 {
    u16 a;
    u16 b;
    u16 c;
    u16 d;
    u32 e;
    u32 f;
};

struct Block1_Entry2 {
    u16 flags;
    u16 id;
    u16 unk1;
    u16 datFile;
    u64 unk2;
};

struct Block2_Entry1 {
    u32 a;
    u32 b;
    u32 c;
    u32 d;
    u32 e;
    u32 f;
    u32 g;
    u32 h;
};

struct ChunkInfo {
    u32 offset;
    u32 length;
    
    u8 data[length] @ offset + parent.header.metaEnd [[sealed,name(std::format("chunkInfo{}", std::core::array_index()))]];
};

struct Block3_Entry1 {
    u16 a;
    u16 b;
};

struct Block3_Entry2 {
    u32 a;
    u32 b;
    u32 c;
};

struct Helpers {
    u32 idList_first @ parent.meta.idList.offset;
    u32 idList_last @ parent.meta.idList.offset + (parent.meta.idList.length-1) * 4;
    
    u64 block1_weird_first @ parent.meta.block1_weird.offset;
    u64 block1_weird_last @ parent.meta.block1_weird.offset + (parent.meta.block1_weird.length-1) * 8;
    
    Block1_Entry1 block1_list1_first @ parent.meta.block1_list1.offset;
    Block1_Entry1 block1_list1_last @ parent.meta.block1_list1.offset + (parent.meta.block1_list1.length-1) * sizeof(Block1_Entry1);
    
    u32 nonZeroData @ parent.meta.block1_list2.offset - 4; // was not in patch02, why between arrays?
    
    Block1_Entry2 block1_list2_first @ parent.meta.block1_list2.offset;
    Block1_Entry2 block1_list2_last @ parent.meta.block1_list2.offset + (parent.meta.block1_list2.length-1) * sizeof(Block1_Entry2);
    
    u64 block2_weird_first @ parent.meta.block2_weird.offset;
    u64 block2_weird_last @ parent.meta.block2_weird.offset + (parent.meta.block2_weird.length-1) * 8;
    
    Block2_Entry1 block2_list1_first @ parent.meta.block2_list1.offset;
    Block2_Entry1 block2_list1_last @ parent.meta.block2_list1.offset + (parent.meta.block2_list1.length-1) * sizeof(Block2_Entry1);
    
    u64 chunkInfo_first @ parent.meta.chunkInfo.offset;
    u64 chunkInfo_last @ parent.meta.chunkInfo.offset + (parent.meta.chunkInfo.length-1) * 8;
    
    u64 block3_weird_first @ parent.meta.block3_weird.offset;
    u64 block3_weird_last @ parent.meta.block3_weird.offset + (parent.meta.block3_weird.length-1) * 8;
    
    Block3_Entry1 block3_list1_first @ parent.meta.block3_list1.offset;
    Block3_Entry1 block3_list1_last @ parent.meta.block3_list1.offset + (parent.meta.block3_list1.length-1) * sizeof(Block3_Entry1);
    
    Block3_Entry2 block3_list2_first @ parent.meta.block3_list2.offset;
    Block3_Entry2 block3_list2_last @ parent.meta.block3_list2.offset + (parent.meta.block3_list2.length-1) * sizeof(Block3_Entry2);
}[[comment("navigation in file")]];


struct KFC {
    Header header;
    Meta meta;
    Helpers helpers;
    
    char comment[meta.comment.length] @ meta.comment.offset;
    DatInfo datInfo[meta.datInfo.length] @ meta.datInfo.offset;
    
    // ignore zeroA and zeroB for now
    One one[meta.one.length] @ meta.one.offset;
    u32 idList[meta.idList.length] @ meta.idList.offset;
    
    // limit large lengths for ImHex
    Weird block1_weird[4000] @ meta.block1_weird.offset;
    Block1_Entry1 block1_list1[4000] @ meta.block1_list1.offset;
    Block1_Entry2 block1_list2[4000] @ meta.block1_list2.offset;
    
    // limit large lengths for ImHex
    Weird block2_weird[4000] @ meta.block2_weird.offset;
    Block2_Entry1 block2_list1[2000] @ meta.block2_list1.offset;
    ChunkInfo chunkInfo[4000] @ meta.chunkInfo.offset; // inline meta files ?

    Weird block3_weird[meta.block3_weird.length] @ meta.block3_weird.offset;
    Block3_Entry1 block3_list1[meta.block3_list1.length] @ meta.block3_list1.offset;
    Block3_Entry2 block3_list2[meta.block3_list2.length] @ meta.block3_list2.offset;
};

KFC kfc @ 0x0;
