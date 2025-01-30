use std::io::{Read, Write};
// use crate::http::response;
// crate는 전체 크레이트의 바깥을 뜻한다.
// 바깥의 http에서 request를 가져와 사용할 수 있다.
use crate::http::{Request, Response, StatusCode, ParseError};
// 트레이트를 범위로 가져와야 사용할 수 있다
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    // trait에서 기본 값으로 제공하는 메소드
    fn handle_bad_request(&mut self, e: &ParseError) -> Response{
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}


// 해당 파일을 통해 분리시키는 방법은
// mod server에 여러가지를 넣어논 것과 같은 상태가 된다
// rust에서 모든 파일은 모듈과 같다.
pub struct Server {
    addr: String,
}
// 구조체에 기능을 추가하기위한 구현 블럭
// 클래스가 있고 클래스안에 모든 기능이 담겨있는 것과 다름
// 메서드, 연관함수가 들어갈 수 있다.
// 메서드는 다른 언어 클래스의 메서드와 비슷하고
// 연관 함수는 다른 언어의 static 메서드와 비슷한 역할을 한다.(구조체의 인스턴스가 필요없다)
// new는 연관함수이다.
// ::구문을 사용해 연관함수에 엑세스 한다.
impl Server {
    //메서드는 항상 첫번째 파라미터로 self를 가져간다
    //self는 구조체의 인스턴스를 나타낸다
    //java나 C++에서는 this로 불린다
    // 해당 코드에서는 run이 구조체의 소유권을 갖는다.
    // 소유권을 갖지 않게하기위해서는 &self로 파라미터를 가지면 된다.
    pub fn run(self, mut handler : impl Handler) {
        println!("Listening on {}", self.addr);
                                                    // 값을 옮기는 것이 아닌, 참조값을 넣어준다
                                                                // result 반환형의 wrap을 벗겨, 결과를 반환시킨다.
                                                                // addr에 연결할 수 없으면 에러를 일으켜 프로그램을 중단시킨다
                                                                // rust의 에러는 2가지 에러가 있다, 에러가 나도 프로그램이 계속 진행되는 에러, 중단하는 에러
        let listener = TcpListener::bind(&self.addr).unwrap();
        
        // while(true){
        // while(true)와 같은 동작을 하는 rust의 특수한 loop이다.
        loop {
            // 'outer과 같은 방식으로 바깥 loop에 대한 참조를 만들 수 있다
            // accepting : TCP에서 매 루프마다 새로운 연결이 있는지 체크하고, 유입되는 연결을 받는것
            // 반환형은 튜플(불변 dto)이며 Result<(TcpStream, SocketAddr)> 이 들어있다
            // let result = listener.accept();

            // if result.is_err() {
            //     continue;
            // }

            // let (stream, addr) = result.unwrap();

            // if - else, switch문을 대신할 수 있는 rust의 표현식 match -> enum타입도 관리할 수 있다, 기본값이나 값의 스킵은 _를 사용한다
            match listener.accept(){
                Ok((mut stream, _)) => {
                     // 클라이언트가 전송한 바이트 읽기
                     // byte는 array형태로 들어오게 된다
                     // rust의 array는 자료형, 크기가 명시적으로 지정되어 스택에 알려줘야한다
                    //  fn arr(a:[u8; 4]){ 같은 형태가 되어야한다 (자료형 u8, 갯수 4개)
                    // 언제나 배열의 길이를 알기 힘들어서 일반적으로 배열을 그대로 넣지 않고 배열의 참조를 넣어 해결할 수 있다. 
                    // 이 경우 배열의 포인터를 알려주는 것이라고 생각하면 된다.
                    // fn arr(a: &[u8]){

                    // 버퍼를 만든 후 초기화
                    // 어레이를 만들고, 초기화 안된 1024개의 배열을 전부 0을 부여해 초기화 시켜 access 가능한 상태로 만든다.
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // 두 방법 모두 byte array를 슬라이스 타입으로 만드는 방법이다.
                            // Request::try_from(&buffer as &[u8]);
                            let response = match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    // dbg!(request);
                                    // Response::new(StatusCode::Ok, Some("<h1>IT WORKS</h1>".to_string()))
                                    // 해당 방식은 heap에 계속해서 할당되고, 서버 실행 동안 메모리가 해제되지 않는 문제가 있다
                                    // response내에서 출력시키고 해제시켜야한다.
                                    // write!(stream, "{}", response);
                                    // response.send(&mut stream);
                                    handler.handle_request(&request)

                                }
                                Err(e) => {
                                    // println!("failed to parse a request: {}", e);
                                    // Response::new(StatusCode::BadRequest, None).send(&mut stream);
                                    // Response::new(StatusCode::BadRequest, None)
                                    handler.handle_bad_request(&e)
                                }
                            };


                            // 기본적으로 response.send가 실행되는데
                            // if let (case) case의 경우 하단의 코드가 실행된다는뜻
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }

                            // 현재 상태론 컴파일러가 추론이 불가능하다. 명확한 자료형을 줘야한다.
                            // &buffer[..].try_into();
                            // let res : &Result<Request, > = &buffer .. 으로 자료형을 줘서 뺴내는 방법도 가능하다.
                        },
                        Err(e) => print!("Failed to read from connection : {}" , e),
                    }
                },
                Err(e)=>println!("Failed to establish a connection: {}", e)
            }

        }

        // 해당 방식으로 함수에서 여러가지 값을 리턴할 수 있다.
        // let tuple = (5, "a", listener);

    }

    // new라는 연관함수 직접 구현
    // 흔히 어떤 구조체의 메인 생성자의 이름을 new라고 하고, 그 모범사례를 따르는게 보통이다.
    // Server를 Self로도 표현이 가능하다.
    pub fn new(addr: String) -> Self {
        Self {
            // addr : addr
            // 필드의 이름이 같다면 생략가능
            addr,
        }
    }
}
