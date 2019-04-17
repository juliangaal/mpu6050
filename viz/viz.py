import numpy as np
import math
import matplotlib.pyplot as plt
import matplotlib.animation as animation


def plot_line(angle: float, length: int):
     '''
     angle - Angle you want your end point at in degrees.
     length - Length of the line you want to plot.

     Will plot the line on a 5 x 5 plot.
     '''

     # unpack the first point
     x, y = (2.5, 2.5)

     # find the end point
     endx = x + length * math.cos(math.radians(angle))
     endy = y + length * math.sin(math.radians(angle))

     # find the start point
     startx = x - length * math.cos(math.radians(angle))
     starty = y - length * math.sin(math.radians(angle))
     
     fig = plt.figure()
     ax = plt.subplot(111)
     ax.set_ylim([0, 5])   # set the bounds to be 10, 10
     ax.set_xlim([0, 5])

     # plot the points
     ax.plot([startx, endx], [starty, endy])
     plt.show()

plot_line(-0.9, 20)
