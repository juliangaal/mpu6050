import numpy as np
import math
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import random
import time

fig = plt.figure()
ax = plt.subplot(111)
ax.set_ylim([0, 5])   # set the bounds to be 10, 10
ax.set_xlim([0, 5])

class RegrMagic(object):
    """Mock for function Regr_magic()
    """
    def __init__(self):
        self.x = 0.0

    def __call__(self) -> float:
        self.x += random.randint(-1, 1)
        return self.x

regr_magic = RegrMagic()

def frames():
    while True:
        yield regr_magic()

def plot_line(angle: float):
     '''
     angle - Angle you want your end point at in degrees.
     length - Length of the line you want to plot.

     Will plot the line on a 5 x 5 plot.
     '''

     # unpack the first point
     x, y = (2.5, 2.5)
     length = 20.0

     # find the end point
     endx = x + length * math.cos(math.radians(angle))
     endy = y + length * math.sin(math.radians(angle))

     # find the start point
     startx = x - length * math.cos(math.radians(angle))
     starty = y - length * math.sin(math.radians(angle))
     ax.clear()

     # plot the points
     return ax.plot([startx, endx], [starty, endy])

ani = animation.FuncAnimation(fig, plot_line, frames=frames, interval=10)
plt.show()
