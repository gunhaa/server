use super::method::Method;
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
