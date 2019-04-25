#!/usr/bin/env python3

import sys
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

def plot(filename: str, filename_2: str, num_data_points: int):
    df_1 = pd.read_csv(filename, delimiter = ',', names=['x', 'y', 'z'])
    df_2 = pd.read_csv(filename_2, delimiter = ',', names=['x', 'y', 'z'])
    n = num_data_points

    y = df_1['x'].to_list()
    var_1 = np.var(y)
    x = np.linspace(0, n, n)
    plt.plot(x, y[:n], label='x raw: var ' + "{0:.5f}".format(var_1), color='black')
    
    y = df_2['x'].to_list()
    var_2 = np.var(y)
    x = np.linspace(0, n, n)
    plt.plot(x, y[:n], label='x calib: var ' + "{0:.5f}".format(var_2), color='red')
    
    plt.ylim([-0.1, 0.1])
    plt.legend()
    plt.show()

    if var_1 < var_2:
        print("Variance of {} is smaller: {}".format(filename, var_1))
    else:
        print("Variance of {} is smaller: {}".format(filename_2, var_2))
        

    y = df_1['y'].to_list()
    var_1 = np.var(y)
    x = np.linspace(0, n, n)
    plt.plot(x, y[:n], label='y raw: var ' + "{0:.5f}".format(var_1), color='black')
    
    y = df_2['y'].to_list()
    var_2 = np.var(y)
    x = np.linspace(0, n, n)
    plt.plot(x, y[:n], label='y calib: var ' + "{0:.5f}".format(var_2), color='red')
    
    plt.ylim([-0.1, 0.1])
    plt.legend()
    plt.show()

    if var_1 < var_2:
        print("Variance of {} is smaller: {}".format(filename, var_1))
    else:
        print("Variance of {} is smaller: {}".format(filename_2, var_2))
        

    y = df_1['z'].to_list()
    var_1 = np.var(y)
    x = np.linspace(0, n, n)
    plt.plot(x, y[:n], label='z raw: var ' + "{0:.5f}".format(var_1), color='black')
    
    y = df_2['z'].to_list()
    var_2 = np.var(y)
    x = np.linspace(0, n, n)
    plt.plot(x, y[:n], label='z calib: var ' + "{0:.5f}".format(var_2), color='red')
    
    plt.ylim([-0.1, 0.1])
    plt.legend()
    plt.show()

    if var_1 < var_2:
        print("Variance of {} is smaller: {}".format(filename, var_1))
    else:
        print("Variance of {} is smaller: {}".format(filename_2, var_2))
        

file_1 = sys.argv[1]
file_1 = sys.argv[1]
file_1 = sys.argv[1]
file_2 = sys.argv[2]
n = int(sys.argv[3])
plot(file_1, file_2, n) 


