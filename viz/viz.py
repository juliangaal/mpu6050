import numpy as np
import sys
import math
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import random
import time

fig = plt.figure()

ax1 = plt.subplot(211)
ax2 = plt.subplot(212)

class RegrMagic(object):
    """Mock for function Regr_magic()
    """
    def __init__(self):
        raw_data = open("sample.txt","r").read()
        self.angles = raw_data.split('\n')
        self.num_data = len(self.angles)
        self.run_index = 0

    def __call__(self) -> float:
        if self.run_index < self.num_data-1:
            x,y = self.angles[self.run_index].split(',')
            self.run_index += 1
            return float(x), float(y)
        else:
            sys.exit(0)

regr_magic = RegrMagic()

def frames():
    while True:
        yield regr_magic()


def get_start_end(point: (float, float), length: float, angle: float):
     x, y = point
     # find the end point
     endx = x + length * math.cos(math.radians(angle))
     endy = y + length * math.sin(math.radians(angle))

     # find the start point
     startx = x - length * math.cos(math.radians(angle))
     starty = y - length * math.sin(math.radians(angle))

     return (endx, endy, startx, starty)

def plot_line(args):
     '''
     angle1 - Angle you want your end point at in degrees.
     length - Length of the line you want to plot.

     Will plot the line on a 5 x 5 plot.
     '''

     # unpack the first point
     x, y = (2., 0.)
     length = 5.0
     roll_angle = args[0]
     pitch_angle = args[1]
     print(roll_angle, pitch_angle)

     ax1.clear()
     ax2.clear()

     ax1.set_ylim([-2, 2])   # set the bounds to be -20, 20
     ax2.set_ylim([-2, 2])   # set the bounds to be -20, 20
     ax1.set_xlim([0, 4])
     ax2.set_xlim([0, 4])

     # plot the points
     endx, endy, startx, starty = get_start_end((x,y), length, roll_angle)
     ax1.plot([startx, endx], [starty, endy], c='black', linewidth='8.0')

     endx, endy, startx, starty = get_start_end((x,y), length, pitch_angle)
     ax2.plot([startx, endx], [starty, endy], c='black', linewidth='8.0')

ani = animation.FuncAnimation(fig, plot_line, frames=frames, interval=10)
# save the animation as an mp4.  This requires ffmpeg or mencoder to be
# installed.  The extra_args ensure that the x264 codec is used, so that
# the video can be embedded in html5.  You may need to adjust this for
# your system: for more information, see
# http://matplotlib.sourceforge.net/api/animation_api.html
#ani.save('double_pendulum.mp4', fps=30, extra_args=['-vcodec', 'libx264'])
plt.legend()
plt.show()
