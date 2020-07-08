
import os
import json
import pathlib

from collections import defaultdict

import pandas

def parse_with_input(path):

    data = {
        "average": defaultdict(list),
        "aerror": defaultdict(list),
        "median": defaultdict(list),
        "merror":  defaultdict(list),
    }
    
    for (path, method, params) in __estimates_path(path):
        values = json.load(open(path))
        
        data["average"][method].append((params, values["mean"]["point_estimate"]))
        data["aerror"][method].append((params, values["mean"]["standard_error"]))
        data["median"][method].append((params, values["median"]["point_estimate"]))
        data["merror"][method].append((params, values["median"]["standard_error"]))

    return data

def __estimates_path(path):
    for method_entry in __generate_dir_entry(path):
        for params_entry in __generate_dir_entry(method_entry.path):
            path = pathlib.Path(params_entry.path) / 'base' / 'estimates.json'

            if path.exists():
                yield (path, method_entry.name, params_entry.name)
                
def __generate_dir_entry(path):
    with os.scandir(path) as it:
        for entry in it:
            if entry.is_dir():
                yield entry
