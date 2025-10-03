# unshrouded

Collection of mods and scripts for dumping data from Enshrouded.

* Mods need **EML/EMM:** https://github.com/Brabb3l/kfc-parser/blob/dev/docs/eml/usage.md
* Scripts are written with **Python 3.13**, but it should work on 3.12 (maybe 3.11) as long as libraries are installed.

## File and folder structure
I have this Git repo checked out into the game folder so that the scripts 
can find the exports from the dumper mod:

```
# this repo
...\Steam\steamapps\common\Enshrouded\mods\dumper\ -- contains mod.json and src
...\Steam\steamapps\common\Enshrouded\scripts\     -- contains python scripts
# will be created
...\Steam\steamapps\common\Enshrouded\export\      -- contains files exported from EML mods
```

## Apply mods without launching the game
* See [export.ps1](export.ps1) 

## Unpack the game into lots of JSON
* See [unpack.ps1](unpack.ps1)
