use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
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

#[derive(Debug, Clone)]
pub struct Token(pub Option<String>);

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

        req.extensions_mut().insert(token);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}
