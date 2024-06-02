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
