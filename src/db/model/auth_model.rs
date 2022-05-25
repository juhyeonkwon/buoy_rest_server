use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Obj {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub idx: i32,
    pub email: String,
    pub password: String,
    pub name: String,
    pub admin: i8,
}

#[derive(Deserialize, Serialize)]
pub struct LoginParam {
    pub email: String,
    pub password: String,
}


#[derive(Deserialize, Serialize)]
pub struct Register {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct OauthRegister {
    pub email: String,
    pub name: String,
    pub sns_type: i8,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Email {
    pub email: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Verify {
    pub email: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Naver {
    pub code : String,
    pub state : String
}
