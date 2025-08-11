use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::HeaderMap,
    Error,
};
use futures_util::future::LocalBoxFuture;
use log::info;
use std::net::IpAddr;
use std::{
    future::{ready, Ready},
    time::Instant,
};

pub struct AccessLog;

impl<S, B> Transform<S, ServiceRequest> for AccessLog
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AccessLogMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AccessLogMiddleware { service }))
    }
}

pub struct AccessLogMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AccessLogMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        let path = req.path().to_string();
        let peer_addr = req.peer_addr();
        let ip_str = parse_ip(req.headers()).unwrap_or_else(|| peer_addr.unwrap().ip().to_string());

        let fut = self.service.call(req);

        Box::pin(async move {
            let response = fut.await?;
            let duration = start.elapsed();

            info!(
                "{} | {} | {} | {}ms | {}",
                method,
                path,
                response.status(),
                duration.as_millis(),
                ip_str
            );

            Ok(response)
        })
    }
}

fn parse_ip(req: &HeaderMap) -> Option<String> {
    let headers = ["X-Forwarded-For", "X-Real-IP", "True-Client-Ip"];

    for header in &headers {
        if let Some(header_value) = req.get(*header) {
            if let Ok(ip_str) = header_value.to_str() {
                for ip in ip_str.split(',') {
                    let ip = ip.trim();
                    if !ip.is_empty() {
                        if let Ok(IpAddr::V4(ipv4)) = ip.parse::<IpAddr>() {
                            return Some(ipv4.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}
