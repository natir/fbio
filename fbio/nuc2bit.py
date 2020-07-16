
from . import criterion

import altair
import pandas
import numpy


def mytheme():
    return {'usermeta': {'embedOptions': {'theme': 'dark'}}, 'config': {'view': {'continuousWidth': 1600, 'continuousHeight': 1200}}}

altair.themes.register('mydark', mytheme)
altair.themes.enable("mydark")

def median_error():
    data = criterion.parse_with_input("target/criterion/nuc2bit")

    data.sort(key=lambda x: int(x[1]))
    
    df = pandas.DataFrame(data, columns=['method', 'len', 'time'])

    df = df.astype({'method': 'str', 'len': 'int64', 'time': 'float64'});

    brush = altair.selection_interval()

    selection = altair.selection_multi(fields=['method'])
    scales = altair.selection_interval(bind='scales')

    color = altair.condition(
        selection,
        altair.Color('method:N', legend=None),
        altair.value('lightgray')
    )
    
    line = altair.Chart(df).mark_line(point=True, size=1).encode(
        x=altair.X('len:Q', title='number of nucleotide'),
        y=altair.Y('median(time):Q', title='time (ns)'),
        color=color,
        tooltip='method:N'
    ).add_selection(
        scales
    )

    error = altair.Chart(df).mark_errorband(extent='iqr').encode(
        x=altair.X('len:Q', title='number of nucleotide'),
        y=altair.Y('time:Q', title='time (ns)'),
        color=color,
        tooltip='method:N'
    )

    legend = altair.Chart(df).mark_point().encode(
        y=altair.Y(
            'method:N',
            axis=altair.Axis(orient='right')
        ),
        color=color
    ).add_selection(
        selection
    )
    
    fig = (line + error) | legend

    fig.add_selection(
        selection
    )
    
    fig.properties(
        width=1600,
        height=1200,
    )
    
    fig.save("nuc2bit.html")
