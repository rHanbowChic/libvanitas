## libvanitas
Python binding for the [whitespace](https://github.com/rHanbowChic/whitespace) library.

### Example
```python
import libvanitas
w = libvanitas.Whitespace("https://note.ms")
w.post("test_ns", "1", "Hello from Python.")
assert w.get("test_ns", "1") == "Hello from Python."
```

```python
import libvanitas
import time
n = libvanitas.Ntms("https://note.ms")
while True:
    n.post("1", "Ho ho ho")
    time.sleep(0.5)
```
### Compiling
```shell
maturin build
```
You need [maturin](https://github.com/PyO3/maturin) to compile the project into a Python wheel.