# Oops: A C++ Race Condition

Let's look at a simple C++ program that has a race condition:


```cpp
#include <thread>
#include <vector>
#include <atomic>

int main() {
  int counter = 0;
  std::vector<std::thread> handles;
    
  for (int i=0; i<3; i++) {
    handles.push_back(
      std::thread([&counter]() { 
        for (int i=0; i<100000; i++) {
          counter++;
        }
      })
    );
  }

  for (int i=0; i<handles.size(); i++) {
    handles[i].join();
  }

  std::cout << "Counter: " << counter << "\n";
  return 0;
}
```

Relatively simple, right? We have three threads incrementing a shared `counter` variable. Unfortunately, there's no synchronization between the threads, so they can read and write to `counter` at the same time. This leads to a race condition, where the final value of `counter` is unpredictable. 

When you run this code, you'll usually get a different answer every time. It compiles without warnings on my system (although sanitizers easily catch this).

When Rust was first created, Mozilla were *really* sick of debugging this! One of Rust's main goals was to prevent this kind of bug from ever happening.