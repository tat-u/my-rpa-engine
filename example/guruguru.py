import rpa_engine as rpa
import math
import time


def calc(t, r):
    return r * math.sin(t * math.pi / 50), r * math.cos(t * math.pi / 50)


r = 200
px, py = calc(0, r)


for t in range(100_000):
    dx, dy = calc(t, r)
    dx -= px
    dy -= py
    rpa.mmv(int(dx), int(dy))
    px, py = px + dx, py + dy
