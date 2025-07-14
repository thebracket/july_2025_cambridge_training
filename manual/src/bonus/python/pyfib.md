# Fibonacci Python

Let's start by working with some slow Python.

Remember we were playing with Fibonacci numbers in Rust? Let's try a Python equivalent:

```python
#/usr/bin/python3
import time

def recur_fibo(n):
   if n <= 1:
       return n
   else:
       return(recur_fibo(n-1) + recur_fibo(n-2))

results = []
t0 = time.time()
for i in range(40):
    results.append(recur_fibo(i))
t1 = time.time()

print(results)
print("Time: ", t1-t0)
```

On my workstation, it finished in 11.176 seconds.