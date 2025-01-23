use actix_web::{dev::Service, dev::ServiceRequest, dev::ServiceResponse, Error as ActixError};
use futures::future::{ready, LocalBoxFuture, Ready};
use log::error;

pub struct ErrorLogging;

impl<S, B> actix_web::dev::Transform<S, ServiceRequest> for ErrorLogging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = ErrorLoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorLoggingMiddleware { service }))
    }
}

pub struct ErrorLoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorLoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().to_string();
        let path = req.path().to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(res) => {
                    if let Some(err) = res.response().error() {
                        let status_code = err.as_response_error().status_code();
                        error!("HTTP Error on {method} {path}: {err}, status code: {status_code}");
                    }
                    Ok(res)
                }
                Err(err) => {
                    error!("{err}");
                    Err(err)
                }
            }
        })
    }
}
