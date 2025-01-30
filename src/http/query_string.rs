use std::collections::HashMap;

// buf와 life time이 같다.
#[derive(Debug)]
pub struct QueryString<'buf> {
    data : HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    // 동적으로 크기를 증가시킬 수 있도록 해야한다.
    // 힙 할당 동적 어레이를 벡터라고 한다.
    // java의 arraylist와 비슷한 역할을 한다.
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}


// 해당 쿼리스트링을 처리할 수 있어야한다
// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf>{
    fn from(s:&'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('='){
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }
            data.entry(key)
            // 중복 키인 경우
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(prev_val) => {
                    // let mut vec = Vec::new();
                    // vec.push(val);
                    // vec.push(prev_val);
                    // 매크로를 이용해 해당 코드를 줄일 수 있다
                    //let mut vec = vec![[prev_val, val]];
                    // existing 포인터를 따라가서 값을 바꾸기(* 사용)
                    *existing = Value::Multiple(vec![prev_val, val]);
                }
                Value::Multiple(vec) => vec.push(val)
            })
            // 새로운 키인 경우
            .or_insert(Value::Single(val));

        }

        QueryString { data }

    }
}