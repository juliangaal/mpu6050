Python script to visualize data, e.g. roll and pitch, used for testing and [demonstration purposes]()

Accepted file format from data recording, e.g.
```
1,2
2,3
3,6
4,9
5,4
6,7
7,7
8,4
9,3
10,7
```

"rate" defines the number of steps from one data point to the next. I.e. if the sensor pumps out 100 measurements a second, it takes way too long to visualize like this. Adjust rate to e.g. 25 to do 25% faster run:
```
$ python3 viz.py 25
```

For requirements, see [requirements.txt](requirements.txt)

On linux, you need to install `tkinter` with
```
$ sudo apt-get install python3-tk
```

If you're having issues with matplotlib, install with apt
```
$ sudo apt-get install python3-matplotlib
```
