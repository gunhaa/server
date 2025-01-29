use super::method::Method;
use std::str::Utf8Error;
use std::str;
// 트레이트를 가져오기
use std::convert::TryFrom;
use std::error::Error;
// use std::fmt::Display;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
// use std::fmt::Formatter;

pub struct Request {
    path: String,

    // 해당 방식으로 query_string의 유무를 알 수 있다.
    // Option은 너무 자주 사용되기에 자동 import된다
    // 만약 import되지 않는다면, 수동으로 use std::option::Option;을 입력해야 할 것이다.
    query_string: Option<String>,
    // query_string : String,
    // 해당 method는 사실 String이 아닌 Enum으로 표현될 수 있다, 가능한 메소드는 정해져 있기 때문이다.
    // Enum은 유한한 값 집합을 갖고 있는 특수한 타입이다.
    // 언어별 enum은 상당히 다른데, Rust의 enum은 haskell의 대수 자료형과 유사하다.
    // JPA에서 String으로 관리하는 것과 비슷함
    // method : super::method::Method,
    // use super::method::Method; 를 통해 생략 할 수 있음
    method: Method,
}

impl Request {

    // Request의 객체로 변환이 불가능하다면?
    // fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
    // }

    // 해당 방식으로도 구현할 수 있지만, 트레이트를 이용해 구현하는 방법이 rust에서 일반적이다.
    // 표준 함수 스타일을 유지해서 코드 유지보수와 추상화에 기여한다
}

// 트레이트를 구현한다
// 트레이트는 다른언어의 인터페이스 , 추상 클래스와 유사한 기능을 한다.
// tryFrom은 실패할수있는 타입 변환에 사용된다.
// 이를 이용해 타입을 확장할 수 있다.
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
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
        let request = str::from_utf8(buf)?;
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
        if c== ' ' {
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