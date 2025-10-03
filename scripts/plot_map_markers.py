from pathlib import Path
import json

import plotly.express as px
import pandas as pd
import numpy as np


def clean_name(name: str) -> str:
    if not name:
        return name
    return (
        repr(name).replace("<location>", "")
        .replace("</location>", "")
        .replace("<lorename>", "")
        .replace("</lorename>", "")
        .replace("<craftingstation>", "")
        .replace("</craftingstation>", "")
        .replace("<npc>", "")
        .replace("</npc>", "")
    )


def main():
    # prepare translation
    with Path("../export/map_markers.json").open(encoding="UTF8") as f:
        map_markers = json.load(f)

    df = pd.json_normalize(map_markers)
    for col in list(df):
        print(col)

    df["name"] = df["name"].apply(lambda name: clean_name(name))

    fig = px.scatter(
        df,
        x="position_wiki.x",
        y="position_wiki.y",
        # x="position.x",
        # y="position.z",
        color="sorting_category",
        hover_data=["name", "template.template_name"],
        symbol="sorting_category",
    )
    fig.update_yaxes(scaleanchor="x", scaleratio=1)
    fig.show()


main()
