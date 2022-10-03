#! /usr/bin/env python3

import scipy
import os
import json

def mean(list):
    return sum(list) / len(list)

bench_out = 'benchmark-output'
json_files = [
    os.path.join(bench_out, f)
    for f in os.listdir(bench_out)
    if f.endswith('json')
]
json_file = max(json_files, key = os.path.getctime)

print(f'Analyzing data from `{json_file}')
print()

with open(json_file, 'r') as f:
    data = json.load(f)

for key in data:
    if key.endswith('ref.rs'):
        continue

    test_name = key.split('/')[-1]
    ref = key.replace('.rs', '_ref.rs')

    new_data = data[key]
    ref_data = data[ref]

    new_mean = mean(new_data) 
    ref_mean = mean(ref_data) 

    change = (new_mean - ref_mean) / ref_mean
    change_100 = change * 100

    test = scipy.stats.ttest_ind(
        new_data, 
        ref_data, 
        equal_var=False, 
        alternative='two-sided'
    )

    print(f'analyzing {test_name}')
    print(f'  {change_100:+05.1f}% in runtime')
    print(f'  {test.pvalue:.4f} p-value')
    print()

print('Note: p-value refers to the Null-Hypothesis being that the measurements are equal')