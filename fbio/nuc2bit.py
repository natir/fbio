
from . import criterion

import plotly.graph_objects as go

def median_error():
    fig = line_with_error_log_xy(get_info("median", "merror"))

    fig.update_layout(template="plotly_dark")
    #fig.update_layout(xaxis_type="log")
    #fig.update_layout(yaxis_type="log")

    fig.show()

    
def average_error():
    fig = line_with_error_log_xy(get_info("average", "aerror"))

    fig.update_layout(template="plotly_dark")
    #fig.update_layout(xaxis_type="log")
    #fig.update_layout(yaxis_type="log")
    
    fig.show()

    
def line_with_error_log_xy(info):
    x, ys, errors = info
    
    fig = go.Figure()

    for method in ys.keys():        
        fig.add_trace(go.Scatter(
            x=x,
            y=ys[method],
            error_y=dict( 
                type='data',
                array=errors[method],
                visible=True
            ),
            mode='lines',
            name=method
        ))

    fig.update_layout(xaxis_title="Number of base");
    fig.update_layout(yaxis_title="Time (ns)");
    fig.update_layout(font=dict(size=18))
    return fig


def get_info(yname, ename):
    data = criterion.parse_with_input("target/criterion/nuc2bit")

    x = [int(val[0]) for val in data[yname][next(iter(data[ename].keys()))]]
    x.sort()

    y = __value_sorted_by_first_int(data, yname)
    error = __value_sorted_by_first_int(data, ename)
    
    return (x, y, error)


def __value_sorted_by_first_int(data, key):
    y = dict()
    
    for method in data[key].keys():
        val = [v[1] for v in sorted(data[key][method], key=lambda x: int(x[0]))]

        y[method] = val

    return y
