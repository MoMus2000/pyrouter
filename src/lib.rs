
use std::{collections::HashMap, io::{Read, Write}, net::{TcpListener, TcpStream}, process::{exit, ExitCode}, thread};
use std::str;

use pyo3::prelude::*;

const BUFFER_SIZE : usize = 8096;

#[pyclass]
#[derive(Clone, Debug)]
struct HttpRouter{
    pub router_elems : Vec<RouterElement>,
    pub mapper: HashMap<String, PyObject>
}

#[pyclass]
#[derive(Clone, Debug)]
struct RouterElement{
    pub path : String,
    pub callback_function: PyObject,
    pub method : String
}

#[pymethods]
impl HttpRouter{
    #[new]
    pub fn new() -> Self{
        HttpRouter{
            router_elems: Vec::<RouterElement>::new(),
            mapper: HashMap::<String, PyObject>::new()
        }
    }

    pub fn parse_and_run_func(&self, http_payload : String) {
        let parsed_url = http_payload;
        self._run_func(parsed_url);
    }

    pub fn add_func(&mut self, path: String, method: String, func: PyObject){
        let router_elem = RouterElement{
            path: path.clone(),
            method: method.clone(),
            callback_function: func.clone()
        };
        self.router_elems.push(router_elem);
        self.mapper.insert(path, func);
    }
}


impl HttpRouter{
    fn _run_func(&self,url: String){ // making it private
        println!("Got this url {url}");
        println!("Length of {}", self.router_elems.len());
        let mapper = self.mapper.clone();
        let func = mapper.get(&url);
        match func {
            Some(value) => {
                unsafe{
                    let py = Python::assume_gil_acquired();
                    value.call1(py, (1, 2));
                }
            }
            _ => {
                println!("Function not found !")
            }
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct SimpleServer{
    pub port : String,
    pub address : String,
    pub router : Option<HttpRouter>
}

#[pymethods]
impl SimpleServer {
    #[new]
    pub fn new(port : String, address: String, router: Option<HttpRouter>) -> Self{
        SimpleServer{
            port,
            address,
            router
        }
    }

    pub fn get_port(&self) -> PyResult<String>{
        Ok(self.port.clone())
    }

    pub fn get_address(&self) -> PyResult<String>{
        Ok(self.address.clone())
    }

    pub fn get_router(&self) -> PyResult<Option<HttpRouter>>{
        Ok(self.router.clone())
    }

    pub fn start_server(&self){
        Python::with_gil(|py| {
            py.run(
                r#"
import signal
print("[python] Ignoring system signals")
signal.signal(signal.SIGINT, signal.SIG_DFL)
            "#,
            None,
            None,
            )
            .unwrap();
        });
        let ip_address = format!("{}{}",self.address, self.port);

        let listener = TcpListener::bind(&ip_address);

        println!("Listening on Ip: {ip_address}");

        match listener {
            Ok(socket) => {
                for stream in socket.incoming() {
                    let router = self.router.as_ref().unwrap().clone();
                    thread::spawn(move || {
                        unsafe{
                            let py = Python::assume_gil_acquired();
                            handle_request(stream.unwrap(), router);
                        }
                    });
                }
            }
            Err(_) => {
                println!("Could not bind to {}", &ip_address);
                exit(1);
            }
        }
        
    }

}

fn handle_request(stream : TcpStream, router: HttpRouter){
    let mut stream = stream.try_clone().unwrap();
    let mut buffer = [0; BUFFER_SIZE];
    println!("> Handling request .. ");
    loop{
        match stream.read(&mut buffer){
            Ok(len) =>{
                let string_result = str::from_utf8(&buffer[0..len]);
                match string_result{
                    Ok(msg) => {
                        println!("> Msg {msg}");
                        router.parse_and_run_func(msg.trim().to_string());
                    }
                    Err(_) => {
                        exit(1);
                    }
                }
            }
            Err(_) => {
                exit(1);
            }
        }
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn pyrouter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SimpleServer>()?;
    m.add_class::<HttpRouter>()?;
    Ok(())
}