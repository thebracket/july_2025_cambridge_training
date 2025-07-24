#/usr/bin/python3
import time
from string_sum import recur_fibo

print("Single thread")
results = []
t0 = time.time()
for i in range(40):
    results.append(recur_fibo(i))
t1 = time.time()

print(results)
print("Time: ", t1-t0)
