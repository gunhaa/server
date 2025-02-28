use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs::{self, canonicalize};

pub struct WebsiteHandler{
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path:String) -> Self {
        Self {public_path}
    }

    fn read_file(&self, file_path:&str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        println!("Canonicalized path: {:?}", fs::canonicalize(&path));
        println!("read_file path : {}", &path);

        let canonicalize_pub_path = fs::canonicalize(&self.public_path).unwrap();

        match fs::canonicalize(path){
            Ok(path) => {
                if path.starts_with(canonicalize_pub_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_)=> None
        }
    }

}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method(){
            Method::GET => match request.path(){
                "/" => {
                    println!("&self.public_path : {}",&self.public_path);
                    Response::new(StatusCode::Ok, self.read_file("index.html"))
                },
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path){
                    Some(contents ) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
                _ => Response::new(StatusCode::NotFound, None),
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}