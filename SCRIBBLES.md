# Scribbles

Unstructured notes, ideas, snippets from other sources.

Pretty much all credits go to the awesome people in the Enshrouded Modding Discord.

---

### Calculate `ResourceId` hash: <sup>(ndoa)</sup>

The kfc_dir itself has the entries by hash of their GUID strings.
If you for example are looking for `3b08d5d5-f556-47b7-8a80-84d6a668d879 b5ce8765 0`,
you would build the string `3b08d5d5-f556-47b7-8a80-84d6a668d879_b5ce8765_0` and hash that.

`0x7a41676800c7dcce: "3b08d5d5-f556-47b7-8a80-84d6a668d879_b5ce8765_0"`

```python
# GUID/UUID is mixed endian!
crc64(f"{guid:str}_{type_name:hex}_{part_index:int}")
```

---

### Calculate `ContentHash` hash:  <sup>(Brabb3l)</sup>

```python
crc64(f"{size:hex}{hash0:hex}{hash1:hex}{hash2:hex}")
```

---

### Calculate `type_name` hash1 from `qualified_name`:  <sup>(ndoa)</sup>

```python
fnv1a_32(b'keen::ds::ecs::ComfortCategory')
```

---

### HashMap in KFC2:

Example for `type_name_hashmap`:

- Length of `type_name_hash` is 117
- Length of `type_name_hashmap` is next larger power of 2: `128`
    - `0b111_1111` as mask for buckets from hash
- since `type_name_hashmap` is `u32`, this is directly used as hash key
- `type_name_hashmap[key & mask]` contains:
    - `index` into `type_name_hash`
    - `count` of collisions, AKA entries to value compare
- iterate `type_name_hash[start:start+length]`, check the hash

Same thing for `content_hashmap`:

- hash key is `ContentHash::hash0`

I do not know