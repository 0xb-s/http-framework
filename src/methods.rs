
use std::error::Error;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
enum MethodHttp {
    GET,
    POST,
    //add more later
}

#[derive(Debug)]
pub struct InvalidMethod;

impl fmt::Display for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid HTTP Method")
    }
}

impl Error for InvalidMethod {}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(MethodHttp);

impl Method {
    pub const GET: Self = Method(MethodHttp::GET);
    pub const POST: Self = Method(MethodHttp::POST);
    //add more later

    pub fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod> {
        match src {
            //add more lter
            b"GET" => Ok(Method(MethodHttp::GET)),
            b"POST" => Ok(Method(MethodHttp::POST)),
            _ => Err(InvalidMethod),
        }
    }

    pub fn is_read_only(&self) -> bool {
        match self.0 {
            MethodHttp::GET | MethodHttp::POST => true,
          // add more later
            _ => false,
        }
    }

    pub fn is_same_result(&self) -> bool {
        todo!()
    }
}
