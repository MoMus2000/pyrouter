# pyrouter

Simple http server concept with rust backend with Python frontend.

## Install

```bash
conda create --name test_pyo3 python=3.10
conda activate test_pyo3
pip3 install maturin
pip3 install pandas
./build.sh

telnet localhost 8080
Then send string(s)
/
/1
```

## Sample Code
```python
import pyrouter
import pandas as pd
import numpy as np

def some_other_func_doing_something():
    d = {'col1': [1, 2], 'col2': [3, 4]}
    df = pd.DataFrame(data=d)
    print(df.head())

def do_something():
    print("Hello World !")
    some_other_func_doing_something()

router = pyrouter.HttpRouter()

router.add_func("/", "GET", do_something)
router.add_func("/1", "GET", some_other_func_doing_something)

simple_server = pyrouter.SimpleServer(":8080", "localhost", router)
simple_server.start_server()
```

## How pyo3 works

```python3
import b
```

Would go look for b.py file and put the contents of the file under the namespace b.

However it does not necessarily have to be the file b.py.

It can also look for a "C extension module" from a compiled library.

On mac and linux these can be b.so or b.pyd on windows.

This is something that the python interpreter publicly supports and has been widely used.

It is used by C, C++, Cython and PyO3.

In pyo3 we use procedural macros (meta programming or decorators) to generate the rust code
for the python C extension modules from annotated rust code.

```rust
#[pyfunction]
/// A Python module implemented in Rust.
//-> Expanded to the rust code needed to create the 
// C extension modules that define what the python 
// functions look like and what they do.
fn my_rust_func(){} 

#[pymodule]
fn pyrouter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SimpleServer>()?;
    m.add_class::<HttpRouter>()?;
    Ok(())
}
```