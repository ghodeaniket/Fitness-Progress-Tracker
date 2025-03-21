use crate::utils::validate_token;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};
use uuid::Uuid;

// JWT auth middleware
pub struct JwtAuth {
    jwt_secret: Rc<String>,
}

impl JwtAuth {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_secret: Rc::new(jwt_secret),
        }
    }
}

// Middleware factory implementation
impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service,
            jwt_secret: self.jwt_secret.clone(),
        }))
    }
}

// Middleware service implementation
pub struct JwtAuthMiddleware<S> {
    service: S,
    jwt_secret: Rc<String>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt_secret = self.jwt_secret.clone();
        let service = self.service.clone();

        Box::pin(async move {
            // Extract the token from the Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| {
                    if h.starts_with("Bearer ") {
                        Some(h[7..].to_string())
                    } else {
                        None
                    }
                });

            // If no token is present, return 401 Unauthorized
            let token = match auth_header {
                Some(t) => t,
                None => {
                    return Err(actix_web::error::ErrorUnauthorized("No token provided"));
                }
            };

            // Validate the token
            match validate_token(&token, &jwt_secret) {
                Ok(claims) => {
                    // Parse the user ID from the token
                    let user_id = match Uuid::parse_str(&claims.sub) {
                        Ok(id) => id,
                        Err(_) => {
                            return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
                        }
                    };
                    
                    // Add the user ID to the request extensions
                    req.extensions_mut().insert(user_id);
                    
                    // Continue with the request
                    service.call(req).await
                }
                Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
            }
        })
    }
}
