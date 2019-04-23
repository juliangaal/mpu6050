#!/usr/bin/env python3

import sys
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

def plot(filename: str, filename_2: str, num_data_points: int):
    df_1 = pd.read_csv(filename, delimiter = ',', names=['roll', 'pitch'])
    df_2 = pd.read_csv(filename_2, delimiter = ',', names=['roll', 'pitch'])
    n = num_data_points

    y = df_1['roll'].to_list()
    x = np.linspace(0, n, n)
    plt.plot(x, y[:n], label='roll raw: var ' + str(np.var(y)), color='black')
    var_1 = np.var(y)
    
    y = df_2['roll'].to_list()
    x = np.linspace(0, n, n)
    plt.plot(x, y[:n], label='roll calibrated: var ' + str(np.var(y)), color='red')
    var_2 = np.var(y)
    
    plt.ylim([-0.1, 0.1])
    plt.legend()
    plt.show()

    if var_1 < var_2:
        print("Variance of {} is smaller: {}".format(filename, var_1))
    else:
        print("Variance of {} is smaller: {}".format(filename_2, var_2))
        

    y = df_1['pitch'].to_list()
    x = np.linspace(0, n, n)
    plt.plot(x, y[:n], label='pitch raw: var ' + str(np.var(y)), color='black')
    var_1 = np.var(y)
    
    y = df_2['pitch'].to_list()
    x = np.linspace(0, n, n)
    plt.plot(x, y[:n], label='pitch calibrated: var ' + str(np.var(y)), color='red')
    var_2 = np.var(y)
    
    plt.ylim([-0.1, 0.1])
    plt.legend()
    plt.show()

    if var_1 < var_2:
        print("Variance of {} is smaller: {}".format(filename, var_1))
    else:
        print("Variance of {} is smaller: {}".format(filename_2, var_2))
        

file_1 = sys.argv[1]
file_1 = sys.argv[1]
file_2 = sys.argv[2]
n = int(sys.argv[3])
plot(file_1, file_2, n) 


