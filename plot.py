#!/usr/bin/env python2

import json
import os
import numpy as np
import matplotlib.pyplot as plt

bar_color = (0 / 255.0, 0 / 255.0, 255 / 255.0, 0.5)
error_bar_color = (255 / 255.0, 0 / 255.0, 0 / 255.0, 0.75)
prefix = ['p', 'n', 'u', 'm', '']


def linear_plot():
    scale = int(np.log(max(median)) / 3 / np.log(10))
    unit = '{}s'.format(prefix[scale + 1])
    scale = 10 ** (3 * scale)
    scaled_median = median / scale
    scaled_err = (err[0] / scale, err[1] / scale)

    plt.barh(pos, scaled_median, align='center', height=0.25, color=bar_color,
             lw=0)
    plt.errorbar(scaled_median, pos, ecolor=error_bar_color, fmt=None,
                 xerr=scaled_err)

    plt.grid(True)
    plt.xlabel('Time ({})'.format(unit))
    plt.ylim(ymax=size)
    plt.yticks(pos, language)

    plt.twinx()
    plt.ylim(ymax=size)
    plt.yticks(pos, relative)

    plt.savefig('plots/{}.png'.format(pid), bbox_inches='tight')
    plt.clf()


def log_plot():
    start = int(np.floor(np.log(min(median)) / np.log(10))) + 3
    end = int(np.ceil(np.log(max(median)) / np.log(10))) + 3

    xs = []
    ticks = []
    for i in range(start, end + 1):
        xs.append(10 ** (i - 3))

        if i % 3 == 0:
            ticks.append('{}s'.format(prefix[i / 3]))
        else:
            ticks.append(str(10 ** (i % 3)))

    plt.barh(pos, median, align='center', height=0.25, left=1e-3,
             color=bar_color, lw=0)
    plt.errorbar(median, pos, ecolor=error_bar_color, fmt=None, xerr=err)

    plt.grid(True)
    plt.xlabel('Time')
    plt.xlim(min(xs), max(xs))
    plt.xscale('log')
    plt.xticks(xs, ticks)
    plt.ylim(ymax=size)
    plt.yticks(pos, language)

    plt.twinx()
    plt.ylim(ymax=size)
    plt.yticks(pos, relative)

    plt.savefig('plots/{}.png'.format(pid), bbox_inches='tight')
    plt.clf()

for path in os.listdir('metrics'):
    pid = path.split('.')[0]

    with open('metrics/' + path) as f:
        metrics = json.loads(f.read())

    size = len(metrics)
    language = map(lambda m: m['language'], metrics)
    median = np.array(map(lambda m: m['median'], metrics))
    upper = np.array(map(lambda m: m['upper'], metrics)) - median
    lower = median - np.array(map(lambda m: m['lower'], metrics))
    err = (lower, upper)
    min_median = min(filter(lambda m: m > 1.0, median))
    relative = map(lambda x: str(round(x / min_median, 2)), median)
    pos = np.arange(size) + 0.5

    if max(median) / min(median) > 1000:
        log_plot()
    else:
        linear_plot()
