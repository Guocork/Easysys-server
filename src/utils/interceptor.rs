//此文件夹用来封装请求以及响应拦截器

use axum::body::Body;
use axum::response::{ IntoResponse, Response};
use axum::http::{ StatusCode};
use tower::Service;
use std::convert::Infallible;

struct ResponseWrapper<S> {
    inner: S,
}

// 实现中间件的 Service trait
impl<S> tower::Service<axum::http::Request<Body>> for ResponseWrapper<S>
where
    S: tower::Service<axum::http::Request<Body>, Response=Result<&'static str, Infallible>>,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = futures::future::Map<S::Future, fn(Result<&'static str, Infallible>) -> Response<Body>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: axum::http::Request<Body>) -> Self::Future {
        let fut = self.inner.call(req);
        fut.map(|result| {
            match result {
                Ok(body) => {
                    Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(body))
                        .unwrap()
                },
                Err(_) => {
                    // Infallible error, this should never happen
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .unwrap()
                }
            }
        })
    }
}