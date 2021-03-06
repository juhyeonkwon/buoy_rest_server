use jsonwebtoken::{decode,  Algorithm, DecodingKey, Validation, /*EncodingKey, Header, encode,*/ };
use serde::{Deserialize, Serialize};

use std::env;
use std::future::{ready, Ready};

use chrono;
use chrono::prelude::*;

use actix_web::error::ErrorUnauthorized;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    // web::ReqData,
    Error, HttpMessage,
};

use futures_util::future::LocalBoxFuture;

pub struct GetUserValue;

impl<S, B> Transform<S, ServiceRequest> for GetUserValue
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = GetUserValueHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(GetUserValueHiMiddleware { service }))
    }
}

pub struct GetUserValueHiMiddleware<S> {
    service: S,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub idx: i32,
    pub email: String,
    pub admin: i8,
    pub exp: usize,
}

impl<S, B> Service<ServiceRequest> for GetUserValueHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let value = req.headers().get("Authorization");

        //Token 값을 가져옵니다.
        let token: Option<String> = match value {
            Some(v) => {
                let _temp = v.to_str().unwrap_or_default();
                let _split: Vec<&str> = _temp.split("Bearer").collect();
                Some(String::from(_split[1].trim()))
            }
            None => None,
        };

        let claim: Option<Claims> = match token {
            Some(v) => get_claim(v),
            None => None,
        };

        if let Some(c) = claim {
            req.extensions_mut().insert(c);

            let fut = self.service.call(req);

            Box::pin(async move {
                let res = fut.await?;

                Ok(res)
            })
        } else {
            Box::pin(async move { Err(ErrorUnauthorized("Unauthorized")) })
        }
    }
}

pub fn get_claim(token: String) -> Option<Claims> {
    let val = Validation::new(Algorithm::HS256);

    let now: DateTime<Local> = Local::now();
    let timestamp = now.timestamp_millis();

    let secret: String = match env::var("SECRET") {
        Ok(v) => v,
        Err(_) => panic!("Env SECRET Not Found!"),
    };

    match decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &val) {
        Ok(v) => {
            if v.claims.exp < timestamp as usize {
                None
            } else {
                Some(v.claims)
            }
        }
        Err(_) => None,
    }
}

// pub fn get_user_claim(token_option: Option<ReqData<Claims>>) -> Claims {
//     if let Some(req_data) = token_option {
//         let claim : Claims = req_data.into_inner();
//         claim
//     } else {
//         Claims {
//             email : "".to_owned(),
//             idx : 0,
//             exp : 0
//         }
//     }
// }
