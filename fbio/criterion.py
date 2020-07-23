
import os
import csv
import random
import pathlib

from collections import defaultdict

import pandas

def parse_with_input(path, subsample_size):

    data = list()
    
    for (path, method, params) in __estimates_path(path):
        with open(path) as fh:
            reader = csv.DictReader(fh)

            local_data = list()
            for (i, raw) in enumerate(reader):
                local_data.append(float(raw["sample_measured_value"])/float(raw["iteration_count"]))

            for val in random.choices(local_data, k=subsample_size):
                data.append((method, params, val))

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
