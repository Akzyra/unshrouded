---
--- Map Helper
---
local map = {}

JSON = require "json"

--- Contains collected data from a map template
---@class MapTemplate
---@field guid Guid
---@field hash u32
---@field template_name string
---@field marker_id_guid Guid?
---@field marker_id_hash u32?
---@field marker_type_guid Guid?
---@field marker_type_hash u32?
---@field loca_tag_guid Guid?
---@field loca_tag_hash u32?
---@field visibility keen.ecs.MapMarkerVisibilityType?

--- Contains all collected marker data
---@class CollectedMarkerData
---@field name string?
---@field sorting_category keen.MarkerTypeSorting?
---@field position {x: u64, y: u64, z: u64}
---@field position_wiki {x: number, y: number}
---@field template MapTemplate


-- Based on those two locations two calculate the factors:
-- ????????????????: 1.5870e13, 5.213e+12
-- 8k_Ancients_HolyFlame_AwakeningChamber_TeleportPoint: 15870086610944, 5211942813696
--  -> Cinder Vault:  0, 0
-- 8kMapLabel_coldHeights_town_05_frozenVillage: 27905079377920, 30071797579776
--  -> Polaris Falls: 2800.00, -5790.00
local offset_x = 15870086610944
local offset_y = 5211942813696
local scale_x = (27905079377920 - 15870086610944) // 2800
local scale_y = (30071797579776 - 5211942813696) // 5790

print("[map] wiki scale X " .. scale_x .. ", Y " .. scale_y)

--- Translate and scale to Wiki coordinates
---@param pos {x: u64, y: u64, z: u64}
---@return {x: u64, y: u64}
local function to_wiki_pos(pos)
    return {
        x = (pos.x - offset_x) // scale_x,
        y = (pos.z - offset_y) // scale_y,
    }
end

--- Get MapMarkerRegistryResource as table index by hash
---
--- @return table<u32, keen.MapMarkerType>
local function get_marker_types()
    ---@type keen.MapMarkerRegistryResource
    local registry = game.assets.get_resources_by_type("keen::MapMarkerRegistryResource")[1].data

    local result = {}
    for _, marker_type in ipairs(registry.mapMarkers) do
        result[marker_type.markerId.value] = marker_type
    end

    return result
end

--- Get all TemplateResource with some kind of map component
---
--- @return table<u32, MapTemplate>
local function get_map_templates()
    local all = game.assets.get_resources_by_type("keen::ecs::TemplateResource")

    local result = {}
    for _, entry in ipairs(all) do
        ---@type keen.ecs.TemplateResource
        local template = entry.data

        ---@type MapTemplate
        local map_template = {
            guid = entry.guid,
            hash = game.guid.hash(entry.guid),
            template_name = template.name,
            marker_id_guid = nil,
            marker_id_hash = nil,
            marker_type_guid = nil,
            marker_type_hash = nil,
            loca_tag_guid = nil,
            loca_tag_hash = nil,
            visibility = nil,
        }
        local is_map_related = false

        -- get data from components, set is_map_related if map component
        for _, variant in ipairs(template.components) do
            local component = variant.value
            if variant.type == "keen::ecs::MapMarker" then
                ---@cast component keen.ecs.MapMarker
                map_template.marker_id_guid = component.markerId
                map_template.marker_type_guid = component.markerType

                map_template.marker_id_hash = game.guid.hash(component.markerId)
                map_template.marker_type_hash = game.guid.hash(component.markerType)

                is_map_related = true
            elseif variant.type == "keen::ecs::LocaTagComponent" then
                ---@cast component keen.ecs.LocaTagComponent
                map_template.loca_tag_guid = component.locaTag
                map_template.loca_tag_hash = game.guid.hash(component.locaTag)
            elseif variant.type == "keen::ecs::MapMarkerVisibilityState" then
                ---@cast component keen.ecs.MapMarkerVisibilityState
                map_template.visibility = component.visibility

                is_map_related = true
            end
        end

        if is_map_related then
            result[map_template.hash] = map_template
        end
    end

    return result
end

--- Dump all map markers to JSON.
---
---@param translation_map table<u32, string> - Hash to Text table
function map.dump_markers(translation_map)
    local json = {}

    local marker_type_lookup = get_marker_types()
    local map_templates = get_map_templates()

    -- handle the scene entity with at least some map markers
    -- looping over all SceneEntityChunkResource is too much data, Lua gives up
    ---@type keen.SceneEntityChunkResource
    local entity_chunk = game.assets.get_resource("509feadb-4c60-425f-9c7c-deeefd9b6920",
        "keen::SceneEntityChunkResource", 1600).data

    for i, entity in ipairs(entity_chunk.entities) do
        --local entity_extra = entity_chunk.entityExtraData[i]

        -- Important: 0-indexed game data to 1-indexed Lua
        local guid = entity_chunk.templates[entity.index + 1]
        local hash = game.guid.hash(guid)

        local map_template = map_templates[hash]
        if not map_template then
            --print("[map] scene entity is not map template: " .. i .. " -> " .. guid .. " (" .. hash .. ")")
            goto continue
        end

        ---@type keen.MapMarkerType
        local marker_type = marker_type_lookup[map_template.marker_type_hash]
        if not marker_type then
            print("[map] no marker_type: " .. i .. " -> " .. guid .. " (" .. hash .. ") ; " .. map_template.template_name)
            --goto continue
        elseif not marker_type.isHighlightable then
            print("[map] not highlightable: " .. i .. " -> " .. guid .. " (" .. hash .. ") ; " .. map_template.template_name)
            --goto continue
        end


        local name = translation_map[map_template.loca_tag_hash] or translation_map[map_template.marker_id_hash] or nil
        local pos = {
            x = entity.transform.position.x,
            y = entity.transform.position.y,
            z = entity.transform.position.z
        }

        ---@type CollectedMarkerData
        local map_template_pos = {
            name = name,
            sorting_category = marker_type and marker_type.sortingCategory or nil,
            position = pos,
            position_wiki = to_wiki_pos(pos),
            template = map_template,
        }
        table.insert(json, map_template_pos)

        --collectgarbage()
        ::continue::
    end

    io.export("map_markers.json", JSON.encode(json))
    print("[map] exported markers JSON")
end

return map
