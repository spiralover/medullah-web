use ntex::http::StatusCode;

pub trait ResponseCodeContract: Clone {
    fn code(&self) -> &str;

    fn status(&self) -> StatusCode;

    fn success(&self) -> bool {
        let code = self.status().as_u16();
        (200..300).contains(&code)
    }

    fn from_code(code: &str) -> Self;

    fn from_status(status: StatusCode) -> Self;
}
