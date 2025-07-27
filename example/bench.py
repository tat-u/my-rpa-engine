import rpa_engine as rpa
import random
import time

start = time.time()

for _ in range(10000):
    time.sleep(0.01)  # Simulate some delay
    dx, dy = random.randint(-10, 10), random.randint(-10, 10)
    rpa.mmv(dx, dy)

print(time.time() - start)
