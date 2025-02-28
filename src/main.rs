// 컴파일러 경고 끄기
#![allow(dead_code)]

use server::Server;
// 파일로 분리시 해당 방법으로 작성 가능하다.
// 컴파일러가 해당 모듈을 찾아 mod server에 넣어준다.
mod server;
use website_handler::WebsiteHandler;
use std::env;

use http::Method;
use http::Request;
mod http;
mod website_handler;


fn main() {


    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];

    let emoji = String::from("🙈❤💚💋");

    let emoji_slice = &emoji[..4];

    let string_borrow: &str = &string;
    
    let string_literal = "1234";


    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);
    dbg!(&emoji);
    dbg!(emoji_slice);

    let default_path = format!("{}/public" , env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path : {}", public_path);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));

}