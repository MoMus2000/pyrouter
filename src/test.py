import pyrouter


def do_same_hara():
    print("Same Hara")

def do_rasengan():
    print("Giant Rasengan")

def do_rasenshuriken():
    print("Rasenshuriken")

def do_something():
    print("Hello World !")

router = pyrouter.HttpRouter()

router.add_func(do_something)
router.add_func(do_rasengan)
router.add_func(do_rasenshuriken)
router.add_func(do_same_hara)

router.run_func(0)
router.run_func(1)
router.run_func(2)
router.run_func(3)

# simple_server = pyrouter.SimpleServer(":8080", "localhost")
# simple_server.start_server()