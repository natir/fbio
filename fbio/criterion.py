
import os
import csv
import pathlib

from collections import defaultdict

import pandas

def parse_with_input(path):

    data = list()
    
    for (path, method, params) in __estimates_path(path):
        with open(path) as fh:
            reader = csv.DictReader(fh)
        
            for (i, raw) in enumerate(reader):
                if i > 50:
                    break
                data.append((method, params, float(raw["sample_measured_value"])/float(raw["iteration_count"])))

    return data

def __estimates_path(path):
    for method_entry in __generate_dir_entry(path):
        for params_entry in __generate_dir_entry(method_entry.path):
            path = pathlib.Path(params_entry.path) / 'base' / 'raw.csv'

            if path.exists():
                yield (path, method_entry.name, params_entry.name)
                
def __generate_dir_entry(path):
    with os.scandir(path) as it:
        for entry in it:
            if entry.is_dir():
                yield entry
