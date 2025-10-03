---
--- Items Helper
---
local items = {}

JSON = require "json"

--- Dump all items to JSON.
--- Credits: based on examples by Brabb3l
---
---@param translation_map table<u32, string>
---@return table<u32, keen.ItemInfo>
function items.dump(translation_map)
    -- ---@type keen.ItemRegistryResource
    -- local registry = game.assets.get_resources_by_type("keen::ItemRegistryResource")[1].data

    local all = game.assets.get_resources_by_type("keen::ItemInfo")

    local json = {}
    local mapping = {}

    for _, res in ipairs(all) do
        -- local res = game.assets.get_resource(item_ref, "keen::ItemInfo")
        -- if res == nil then
        --     print("[items] ERR: missing item info for " .. tostring(item_ref))
        --     goto continue
        -- end

        ---@type keen.ItemInfo
        local item = res.data

        local name = translation_map[game.guid.hash(item.name)] or nil
        local description = translation_map[game.guid.hash(item.description)] or nil
        local lore = translation_map[game.guid.hash(item.lore)] or nil

        table.insert(json, {
            guid = res.guid,
            id = item.itemId.value,
            debugName = item.debugName,
            name = name,
            description = description,
            lore = lore,
            rarity = item.rarity,
            maxStackSize = item.maxStackSize,
            category = item.category,
            generateRarity = item.generateRarity,
            randomLootStackRange = {
                minStackSize = item.randomLootStackRange.minStackSize,
                maxStackSize = item.randomLootStackRange.maxStackSize,
            },
        })

        mapping[game.guid.hash(res.guid)] = item

        ::continue::
    end

    io.export("items.json", JSON.encode(json))
    print("[items] exported JSON")
    return mapping
end

---@param translation_map table<u32, string>
---@return table<u32, string>
local function get_workshop_names(translation_map)
    ---@type keen.WorkshopRegistryResource
    local registry = game.assets.get_resources_by_type("keen::WorkshopRegistryResource")[1].data
    local mapping = {}

    for _, npc in ipairs(registry.npcs) do
        mapping[npc.workshopId.value] = translation_map[npc.name.value]
    end

    for _, workshop in ipairs(registry.workshops) do
        mapping[workshop.workshopId.value] = translation_map[workshop.name.value]
    end

    for _, prop in ipairs(registry.craftingProps) do
        mapping[prop.craftingPropId.value] = translation_map[prop.name.value]
    end
    return mapping
end

--- Dump all recipes to JSON.
---
---@param items_map table<u32, keen.ItemInfo>
---@param translation_map table<u32, string>
function items.dump_recipes(items_map, translation_map)
    ---@type keen.RecipeRegistryResource
    local registry = game.assets.get_resources_by_type("keen::RecipeRegistryResource")[1].data
    local json = {}

    local workshop_names = get_workshop_names(translation_map)

    for _, recipe in ipairs(registry.recipes) do
        local inputs = {}
        local outputs = {}

        for _, entry in ipairs(recipe.input) do
            local item = items_map[entry.item.value]
            table.insert(inputs, {
                name = translation_map[game.guid.hash(item.name)],
                count = entry.count,
                -- guid = entry.itemRef,
                -- hash = entry.item.value,
            })
        end

        for _, entry in ipairs(recipe.output) do
            local item = items_map[entry.item.value]
            table.insert(outputs, {
                name = translation_map[game.guid.hash(item.name)],
                count = entry.count,
                -- guid = entry.itemRef,
                -- hash = entry.item.value,
            })
        end

        table.insert(json, {
            inputs = inputs,
            outputs = outputs,
            craft_at = workshop_names[recipe.workshopId.value],
            requires_sheltered = recipe.requiresSheltered,
            duration = recipe.craftingDuration.value // 1e9, -- convert nanoseconds
        })
    end

    io.export("recipes.json", JSON.encode(json))
    print("[recipes] exported JSON " .. #json)
end

return items
