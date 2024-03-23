import pyrouter
import pandas as pd
import numpy as np

def some_other_func_doing_someshit():
    d = {'col1': [1, 2], 'col2': [3, 4]}
    df = pd.DataFrame(data=d)
    print(df.head())

def do_same_hara():
    print("Same Hara")
    some_other_func_doing_someshit()

def do_rasengan():
    print("Giant Rasengan")

def do_rasenshuriken():
    print("Rasenshuriken")

def do_something():
    print("Hello World !")

router = pyrouter.HttpRouter()

router.add_func("/", "GET", do_something)
router.add_func("/1", "GET", do_rasengan)
router.add_func("/2", "GET", do_rasenshuriken)
router.add_func("/3", "GET", do_same_hara)

simple_server = pyrouter.SimpleServer(":8080", "localhost", router)
simple_server.start_server()