// enum을 str로 바꿀때 사용하는 특수한 모듈
use std::str::FromStr;
#[derive(Debug)]
pub enum Method {
    // 메모리에서 Enum은 단순한 숫자로 표시된다,
    // GET으로 온다면 0, 이후부턴 1씩 증가하는 식으로 표현된다.
    // 요청은 query_string을 받기 때문에 실제로 String을 가진다고 볼 수 있다.
    GET,
    // delete는 db로 지운다는 명령을 줘야하기때문에 u64를 가진다
    // rust enum의 장점은 다른 유형의 데이터를 담을 수 있다는 것이다
    DELETE,
    POST,
    PUT,
    // 이렇게 바꾸게되면, 앞부분은 똑같이 0,1,2 가 되고, put은 5가되고 이후부터는 6,7,8... 으로 진행되게 된다.
    // PUT = 5,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET), 
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError)
        }
    }
}

pub struct MethodError;