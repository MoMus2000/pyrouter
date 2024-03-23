
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, process::{exit, ExitCode}, thread};
use std::str;

use pyo3::prelude::*;

const BUFFER_SIZE : usize = 8096;

#[pyclass]
#[derive(Clone)]
struct HttpRouter{
    pub router_elems : Vec<RouterElement>
}

#[pyclass]
#[derive(Clone)]
struct RouterElement{
    pub path : &'static str,
    pub callback_function: PyObject,
    pub method : &'static str
}

#[pymethods]
impl HttpRouter{
    #[new]
    pub fn new() -> Self{
        HttpRouter{
            router_elems: Vec::<RouterElement>::new()
        }
    }

    pub fn parse_payload(&self, http_payload : String) -> String{
        "/".to_string()
    }

    pub fn add_func(&mut self, func: PyObject){
        let router_elem = RouterElement{
            path: "/",
            method: "GET",
            callback_function: func
        };
        self.router_elems.push(router_elem);
    }
    
    pub fn run_func(&self, index: usize){
        if index >= self.router_elems.len() {
            return
        }
        let _ = Python::with_gil(|py| {
            self.router_elems[index].callback_function.clone().call0(py).unwrap();
        });
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
        let ip_address = format!("{}{}",self.address, self.port);

        let listener = TcpListener::bind(&ip_address);

        println!("Listening on Ip: {ip_address}");

        match listener {
            Ok(socket) => {
                for stream in socket.incoming(){
                    let router = self.router.as_ref().unwrap().clone();
                    thread::spawn(move || {
                        handle_request(stream.unwrap(), router);
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
    loop{
        match stream.read(&mut buffer){
            Ok(len) =>{
                let string_result = str::from_utf8(&buffer[0..len]);
                match string_result{
                    Ok(msg) => {
                        let route = router.parse_payload(msg.to_string());
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
