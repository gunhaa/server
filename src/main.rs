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
    // let string_slice = &string[10..14];
    // 같은 역할을 한다. rust는 문자를 모두 가져온다
    // String의 heap은 시작지점의 ptr, length, capacity를 가진다
    // &str의 경우 length는 알고, ptr의 위치만 정해주면 되기 때문에 해당 구문이 성립한다.
    // 러스트의 모든 문자열은 UTF-8로 인코딩되어있다.
    // 즉, 한 문자가 1바이트인지 확실하지 않기때문에 해당 방식은 좋은 방식이 아니다.
    // 해당 10.. 은 10바이트 뒤의 모든 문자를 달라는 의미와 같아, 문제가 생길수있다.
    let string_slice = &string[10..];

    // 해당 예제에서는 문제가 된다
    let emoji = String::from("🙈❤💚💋");

    // 4개를 자를려고했지만, 각 이모지는 하나의 이모지가 4바이트를 차지하기때문에 한개만 가져오게된다
    let emoji_slice = &emoji[..4];

    // String을 받아와, 자동으로 전체를 자른 string slice가 된다.
    let string_borrow: &str = &string;
    
    // 해당 문자열은 immutable하다, 컴파일링 할때 지정해서 크기가 알려진 상태이기 때문이다.
    let string_literal = "1234";

    // 해당 방식은 dbg!에 소유권을 부여하는 방식이고,
    // 해당 코드는 소유권을 이전하려고 하기 때문에 컴파일 오류가 발생한다
    // dbg!(string);
    // 참조를 주는 것으로 해결 할 수 있다.
    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);
    dbg!(&emoji);
    dbg!(emoji_slice);

                // 만들려는 Server의 인터페이스
                // Server는 구조체이다.
                // Rust의 구조체는 사용자 지정 데이터 타입이다.
                // 이를 이용해 관련 데이터를 한 곳에 모을 수 있다.
                // 객체 지향 언어의 클래스와 비슷한 역할을 한다.

    // let get = Method::GET("abcd".to_string());
    // let delete = Method::DELETE(100);
    // let post = Method::POST;
    // let put = Method::PUT;

    // 메소드 GET에서 없음, 있음을 rust에서 표현하는 방법?
    // rust에는 null 이 존재하지 않는다
    // 대신 표준 라이브러리 Enum Option을 사용한다
    // Option 은 두가지 상태 None, Some(T)가 존재한다.
    // T는 제네릭 타입을 의미한다. 어떤 타입도 담을 수 있지만, 명시적으로 지정되어야 한다.
    // let server = server::Server::new("127.0.0.1:8080".to_string());
    // use Server::server로 생략할 수 있음

    // 따로 파일이 없다면 PUBLIC_PATH=$(pwd)/public cargo run 으로 실행 가능하다
    // 컴파일러에 저장된 현재 디렉토리
    // cargo expand | code - 로 확인 가능
    let default_path = format!("{}/public" , env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path : {}", public_path);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));

}

// // server 모듈 생성, 일종의 server 네임스페이스 안에 있는 것과 같다.
// // 안에 들어있는 모든 값은 기본적으로 private를 가진다.
// // 사용하고 싶다면 모두 public으로 열어야 한다.(pub 사용)
// mod server{
//     // Server 구조체 생성
//     pub struct Server {
//         addr: String, 
//     }
//     // 구조체에 기능을 추가하기위한 구현 블럭
//     // 클래스가 있고 클래스안에 모든 기능이 담겨있는 것과 다름
//     // 메서드, 연관함수가 들어갈 수 있다.
//     // 메서드는 다른 언어 클래스의 메서드와 비슷하고
//     // 연관 함수는 다른 언어의 static 메서드와 비슷한 역할을 한다.(구조체의 인스턴스가 필요없다)
//     // new는 연관함수이다.
//     // ::구문을 사용해 연관함수에 엑세스 한다.
//     impl Server {
//         //메서드는 항상 첫번째 파라미터로 self를 가져간다
//         //self는 구조체의 인스턴스를 나타낸다
//         //java나 C++에서는 this로 불린다
//         // 해당 코드에서는 run이 구조체의 소유권을 갖는다.
//         // 소유권을 갖지 않게하기위해서는 &self로 파라미터를 가지면 된다.
//         pub fn run(self){
//             println!("Listening on {}", self.addr);
//         }

//         // new라는 연관함수 직접 구현
//         // 흔히 어떤 구조체의 메인 생성자의 이름을 new라고 하고, 그 모범사례를 따르는게 보통이다.
//         // Server를 Self로도 표현이 가능하다.
//         pub fn new(addr : String) -> Self{
//             Self {
//                 // addr : addr
//                 // 필드의 이름이 같다면 생략가능
//                 addr
//             }
//         }
//     }
// }

// mod http {

//     pub mod request {
//         use super::method::Method;
//         pub struct Request {
//             path : String,
        
//             // 해당 방식으로 query_string의 유무를 알 수 있다.
//             // Option은 너무 자주 사용되기에 자동 import된다
//             // 만약 import되지 않는다면, 수동으로 use std::option::Option;을 입력해야 할 것이다.
//             query_string : Option<String>,
//             // query_string : String,
//             // 해당 method는 사실 String이 아닌 Enum으로 표현될 수 있다, 가능한 메소드는 정해져 있기 때문이다.
//             // Enum은 유한한 값 집합을 갖고 있는 특수한 타입이다.
//             // 언어별 enum은 상당히 다른데, Rust의 enum은 haskell의 대수 자료형과 유사하다.
//             // JPA에서 String으로 관리하는 것과 비슷함
//             // method : super::method::Method,
//             // use super::method::Method; 를 통해 생략 할 수 있음
//             method : Method,
//         }
//     }

//     pub mod method {
//         pub enum Method {

//             // 메모리에서 Enum은 단순한 숫자로 표시된다,
//             // GET으로 온다면 0, 이후부턴 1씩 증가하는 식으로 표현된다.
//             // 요청은 query_string을 받기 때문에 실제로 String을 가진다고 볼 수 있다.
//             GET,
//             // delete는 db로 지운다는 명령을 줘야하기때문에 u64를 가진다
//             // rust enum의 장점은 다른 유형의 데이터를 담을 수 있다는 것이다
//             DELETE,
//             POST,
//             PUT,
//             // 이렇게 바꾸게되면, 앞부분은 똑같이 0,1,2 가 되고, put은 5가되고 이후부터는 6,7,8... 으로 진행되게 된다.
//             // PUT = 5,
//             HEAD,
//             CONNECT,
//             OPTIONS,
//             TRACE,
//             PATCH,
//         }
//     }


// }




/* 일반적인 HTTP 요청
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/