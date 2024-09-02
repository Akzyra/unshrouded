# unshrouded

**Very WIP** tool for unpacking the files of the game Enshrouded.
Currently only reading KFC meta-data.

## Basic overview of data layout

*The "resource" and "content" names are based on `keen::ResourceId` and `keen::ContentHash` from reflection dumps.*

- One `.kfc` File:
  - header
  - meta-data about the array data
  - array data:
    - comment / fileversion string
    - `.dat` file info: size and content count
    - meta-data to locate internal "resource" data
    - meta-data to locate external "content" blocks
    - data about types
    - "resource" blocks inline until end
      - identified by `keen::ResourceId`
- Multiple `.dat` Files:
  - "content" entries split over currently 32 files
    - identified by `keen::ContentHash`
