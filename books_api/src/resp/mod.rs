use poem_openapi::{payload::Json, ApiResponse, Object};

#[derive(Object)]
pub struct RespData {
    code: i8,
    msg: Option<String>,
}

impl RespData {
    pub fn new(code: i8, msg: Option<String>) -> Self {
        RespData { code, msg }
    }
}

#[derive(ApiResponse)]
pub enum Response<T, E>
where
    T: std::marker::Send + poem_openapi::types::Type + poem_openapi::types::ToJSON,
    E: std::marker::Send + poem_openapi::types::Type + poem_openapi::types::ToJSON,
{
    #[oai(status = 200)]
    Ok(Json<T>),

    #[oai(status = 404)]
    NotFound(Json<E>),

    #[oai(status = 409)]
    AlreadyExists(Json<E>),

    #[oai(status = 500)]
    InternalError(Json<E>),
}

impl<T, E> Response<T, E>
where
    T: std::marker::Send + poem_openapi::types::Type + poem_openapi::types::ToJSON,
    E: std::marker::Send + poem_openapi::types::Type + poem_openapi::types::ToJSON,
{
    pub fn ok(data: T) -> Self {
        Response::Ok(Json(data))
    }

    pub fn not_found(data: E) -> Self {
        Response::NotFound(Json(data))
    }

    pub fn already_exists(data: E) -> Self {
        Response::AlreadyExists(Json(data))
    }

    pub fn internal_error(data: E) -> Self {
        Response::InternalError(Json(data))
    }
}
