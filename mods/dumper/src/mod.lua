RUN_DUMPER = true

if not RUN_DUMPER then
    return
elseif not loader.features.export then
    print("Export not enabled")
    return
end

TRANSLATIONS = require "translations"
ITEMS = require "items"
MAP = require "map"

-- load and dump some languages
local translation_map_none = TRANSLATIONS.get_mapping()
local translation_map_en_us = TRANSLATIONS.get_mapping("En_Us")
local translation_map_de_de = TRANSLATIONS.get_mapping("De_De")

-- language used for dumping other data
local translation_map = translation_map_en_us

-- dump items
local items_map = ITEMS.dump(translation_map)
ITEMS.dump_recipes(items_map, translation_map)

-- dump map stiff
MAP.dump_markers(translation_map)
