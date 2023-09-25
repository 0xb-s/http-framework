use std::error::Error;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
enum MethodHttp {
    GET,
    POST,
    OPTION,
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
    pub const OPTION: Self = Method(MethodHttp::OPTION);

    pub fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod> {
        match src {
            b"GET" => Ok(Method(MethodHttp::GET)),
            b"POST" => Ok(Method(MethodHttp::POST)),
            b"OPTION" => Ok(Method(MethodHttp::OPTION)),
            _ => Err(InvalidMethod),
        }
    }

    pub fn is_read_only(&self) -> bool {
        match self.0 {
            MethodHttp::GET | MethodHttp::OPTION => true,
            _ => false,
        }
    }

    pub fn is_same_result(&self) -> bool {
        todo!()
    }
}
