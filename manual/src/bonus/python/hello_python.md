# Saying Hello to Python

> See https://pyo3.rs/v0.24.2/

Start by making a virutal environment with `maturin` inside:

```bash
# (replace string_sum with the desired package name)
mkdir string_sum
cd string_sum
python -m venv .env
source .env/bin/activate
pip install maturin
maturin init --bindings pyo3
```

(And select `pyo3`)

Then run the example:

```bash
$ maturin develop
# lots of progress output as maturin runs the compilation...
$ python
>>> import string_sum
>>> string_sum.sum_as_string(5, 20)
'25'
```