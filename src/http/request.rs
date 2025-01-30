use super::method::{Method, MethodError};
use std::str::Utf8Error;
use std::str;
// 트레이트를 가져오기
use std::convert::TryFrom;
use std::error::Error;
// use std::fmt::Display;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use super::{QueryString, QueryStringValue};
// use std::fmt::Formatter;
                // lifetime 설정방법
// 기본 트레이트 설정을 적용한다.
#[derive(Debug)]
pub struct Request<'buf> {
    // buffer를 참조하고있기때문에 dangling pointer가 된다(use after free)
    // 그래서 life cycle을 필요로 한다.
    // 라이프 사이클이 없다면 Request의 수명이 버퍼보다 긴 상황이여서 문제가 생긴다.
    // GC가 있는 고수준의 언어에서는 문제가 될 일이 없는 일이다
    // 고수준의 언어들은 GC가 참조값이 존재한다면 할당을 유지한다.
    // Rust는 컴파일러가 dangling pointer 문제를 막아줘서, 컴파일이 된다면 dangling pointer문제가 없다
    // 즉, 런타임에 성능이 희생당하는 문제가 없다.
    // life cycle은 Rust언어만이 가지고있는 독특한 특징이다.
    path: &'buf str,

    // 해당 방식으로 query_string의 유무를 알 수 있다.
    // Option은 너무 자주 사용되기에 자동 import된다
    // 만약 import되지 않는다면, 수동으로 use std::option::Option;을 입력해야 할 것이다.
    query_string: Option<QueryString<'buf>>,
    // query_string : String,
    // 해당 method는 사실 String이 아닌 Enum으로 표현될 수 있다, 가능한 메소드는 정해져 있기 때문이다.
    // Enum은 유한한 값 집합을 갖고 있는 특수한 타입이다.
    // 언어별 enum은 상당히 다른데, Rust의 enum은 haskell의 대수 자료형과 유사하다.
    // JPA에서 String으로 관리하는 것과 비슷함
    // method : super::method::Method,
    // use super::method::Method; 를 통해 생략 할 수 있음
    method: Method,
}

// getter
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// impl Request {

    // Request의 객체로 변환이 불가능하다면?
    // fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
    // }

    // 해당 방식으로도 구현할 수 있지만, 트레이트를 이용해 구현하는 방법이 rust에서 일반적이다.
    // 표준 함수 스타일을 유지해서 코드 유지보수와 추상화에 기여한다
// }

// 트레이트를 구현한다
// 트레이트는 다른언어의 인터페이스 , 추상 클래스와 유사한 기능을 한다.
// tryFrom은 실패할수있는 타입 변환에 사용된다.
// 이를 이용해 타입을 확장할 수 있다.
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // 결국 수명은 자료형의 일부분이다.
    // 컴파일러의 추론과정과 범위를 이해해야 제대로 쓸 수 있다.
    // 수명은 우리가 컴파일러에게 제공하는 메타데이터와 비슷하다.
    
    // 결국 수명이란 RUST컴파일러가 메모리 안전을 보장할 수 있게 해주는 도구이다. 
    // 수명은 메모리에 관련되어 동일한 수명을 공유할 것으로 예상된다는 것을 컴파일러에게 알려주도록 한다.
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // match str::from_utf8(buf){
        //     Ok(request) => {

        //     },
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }
        // 위 코드와 같은 동작을 더 짧고 편하게 구현할 수 있다
        // 정상적으로 동작하면 Result가 반환되고, 에러시 동작을 or을 통해 설정할 수 있다.
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)){
        //     Ok(request) => {}
        //     Err(e)=> return Err(e),
        // }
        // 해당 패턴은 rust에서 많이 사용되는 패턴으로, 특수 구문을 통해서도 사용 할 수 있다.
        // 에러가 아니라면 Result가 unwrap되어 반환되고, 에러시 or구문에서 설정된 에러가 던져진다
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        // 해당 방식으로하면 자동으로 함수의 에러가 ParseError로 던져진다.
                        // 해당 함수는 유효한 UTF-8인지 검사하고 &str로 변환(해석)할 뿐이다.
        let request = str::from_utf8(buf)?;

        // 받을 요청
        // GET /search?name=abc&sort=1 HTTP/1.1
        // match get_next_word(request){
        //     Some((method, request)) => {},
        //     None => return Err(ParseError::InvalidRequest),
        // }
        // 위 코드와 같은 동작을 한다
        // 같은 변수를 사용하는것을 섀도잉이라고하며, 값을 덮어쓴다.
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        // match path.find('?') {
        //     Some(i) => {
        //                     // ? 제거, ?는 1byte
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i+1..]);
        //     path = &path[..i];
        // }
        // 위 코드의 개선 형태이다.
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        Ok(Self{
            // 문자열 슬라이스를 실제 문자열로 변환
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // iterator 반환
    let mut iterator = request.chars();
    // loop {
    //     let item = iterator.next();
    //     match item {
    //         Some(c) => {},
    //         None => break,
    //     }
    // }
    // 해당 코드 대신 for을 이용해 iterator를 사용할 수 있다.
    // 해당 방식으로 index도 사용해 튜플로 반환할 수 있다
    for (i,c) in iterator.enumerate(){
            // 공백, 혹은 캐리지 리턴
        if c== ' ' || c=='\r' {
            // 해당 i+1방식은, RUST에서는 한글자 다음의 인덱스가 아닌 1바이트 이후를 사용하는 것이라 매우 위험하다.
            // 하지만 이 경우 공백은 1바이트라는 것이 자명하기때문에 +1을 사용해도 안전하다.
            return Some((&request[..i], &request[i+1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}



impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Enconding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self{
        Self::InvalidMethod
    }   
}


impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }   
}

impl Error for ParseError {
    
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}