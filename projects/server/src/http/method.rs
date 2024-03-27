pub enum Method {
    GET(String), //0  
    DELETE(u64), //1
    POST, //2  //if we set POST = 5, then everything after would be 6, 7, 8, ...
    PUT, //....
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}