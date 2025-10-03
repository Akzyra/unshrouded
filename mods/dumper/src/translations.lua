---
--- Translations Helper
---
local translations = {}

JSON = require "json"

--- Dumps `language_id` and returns a mapping of ID to Text.
--- Returns a mapping of ID to Translation for given language.
--- Credits: based on examples by Brabb3l
---
---@param language_id keen.LanguageId? -- if not provied falls back to "keenglish"
---@return table<u32, string>
function translations.get_mapping(language_id)
    ---@type keen.LocaTagCollectionResource
    local collection = game.assets.get_resources_by_type("keen::LocaTagCollectionResource")[1].data
    local dataHash = nil

    if language_id == nil then
        language_id = "None"
        dataHash = collection.keenglishDataHash
    else
        for _, language in ipairs(collection.languages) do
            if language.language == language_id then
                dataHash = language.dataHash
                break
            end
        end
    end

    if dataHash == nil then
        error("[translations] cannot get dataHash for" .. language_id)
    end

    local guid = game.guid.from_content_hash(dataHash)
    local buffer = game.assets.get_content(guid):read_data()

    ---@type keen.LocaTagCollectionResourceData
    local collectionData = buffer:read_resource("keen::LocaTagCollectionResourceData")

    local json = {
        guid = guid,
        tags = {}
    }
    local mapping = {}

    for _, tag in ipairs(collectionData.tags) do
        mapping[tag.id.value] = tag.text

        -- Recreate userdata as basic tables to JSON works
        local arguments = {}

        for _, arg in ipairs(tag.arguments) do
            table.insert(arguments, {
                id = arg.id,
                type = arg.type
            })
        end

        table.insert(json.tags, {
            id = tag.id.value,
            text = tag.text,
            arguments = arguments,
            genericArguments = tag.genericArguments,
        })
    end

    io.export(
        "translation_" .. tostring(language_id) .. ".json",
        JSON.encode(json)
    )
    print("[translations] exported JSON for " .. tostring(language_id))

    return mapping
end

return translations
