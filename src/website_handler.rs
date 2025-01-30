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
        // result가 문제없다면 option으로 변환한다, Some()이 된다.
        // result가 오류라면 none으로 변환한다
        // fs::read_to_string(path).ok()
        // 해당 방식은 공격에 굉장한 취약성을 가지고 있다
        // ../../ 방식으로 모든 것을 볼 수 있게 된다.

        println!("Canonicalized path: {:?}", fs::canonicalize(&path));
        println!("read_file path : {}", &path);

        // Windows에서 fs::canonicalize를 사용하면 \\?\(UNC 경로 프리픽스)가 붙으면서 경로가 정규화되서 문제가 생긴다
        // 일치시켜 문제를 해결해야한다.
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
        // Response::new(StatusCode::Ok, Some("<h1>Test</h1>".to_string()))
        match request.method(){
            Method::GET => match request.path(){
                "/" => {
                    println!("&self.public_path : {}",&self.public_path);
                    // C:\workspace\git\Rust\http_server\src\public\hello.html
                    // println!("serving item :{}", self.read_file("index.html").unwrap_or("File not found".to_string()));
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