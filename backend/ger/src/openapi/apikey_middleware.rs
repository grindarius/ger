use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    HttpResponse,
};
use futures::future::LocalBoxFuture;

use crate::{errors::HttpError, constants::{SWAGGER_API_KEY_NAME, SWAGGER_API_KEY}};

pub struct ApiKeyMiddleware<S> {
    pub service: S,
    pub log_only: bool,
}

impl<S> Service<ServiceRequest> for ApiKeyMiddleware<S>
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<actix_web::body::BoxBody>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    fn poll_ready(
        &self,
        context: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(context)
    }

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let response = |request: ServiceRequest, response: HttpResponse| -> Self::Future {
            Box::pin(async { Ok(request.into_response(response)) })
        };

        match request.headers().get(SWAGGER_API_KEY_NAME.to_string()) {
            Some(key) if key != SWAGGER_API_KEY.as_str() => {
                if self.log_only {
                    tracing::debug!("incorrect api provided")
                } else {
                    return response(
                        request,
                        HttpResponse::Unauthorized()
                            .json(HttpError::InvalidSwaggerAPIKey.get_error_struct()),
                    );
                }
            }
            None => {
                if self.log_only {
                    tracing::debug!("missing api key")
                } else {
                    return response(
                        request,
                        HttpResponse::Unauthorized()
                            .json(HttpError::InvalidSwaggerAPIKey.get_error_struct()),
                    );
                }
            }
            // passthrough
            _ => (),
        }

        if self.log_only {
            tracing::debug!("performing operation")
        }

        let future = self.service.call(request);

        Box::pin(async move {
            let response = future.await?;
            Ok(response)
        })
    }
}
