// @generated
/// Generated client implementations.
pub mod user_schema_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct UserSchemaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserSchemaServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UserSchemaServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UserSchemaServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            UserSchemaServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn list_user_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUserSchemasResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.schema.v3alpha.UserSchemaService/ListUserSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.schema.v3alpha.UserSchemaService",
                        "ListUserSchemas",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** User schema by ID

 Returns the user schema identified by the requested ID.
*/
        pub async fn get_user_schema_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserSchemaByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserSchemaByIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.schema.v3alpha.UserSchemaService/GetUserSchemaByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.schema.v3alpha.UserSchemaService",
                        "GetUserSchemaByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Create a user schema

 Create the first revision of a new user schema. The schema can then be used on users to store and validate their data.
*/
        pub async fn create_user_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUserSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateUserSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.schema.v3alpha.UserSchemaService/CreateUserSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.schema.v3alpha.UserSchemaService",
                        "CreateUserSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Update a user schema

 Update an existing user schema to a new revision. Users based on the current revision will not be affected until they are updated.
*/
        pub async fn update_user_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.schema.v3alpha.UserSchemaService/UpdateUserSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.schema.v3alpha.UserSchemaService",
                        "UpdateUserSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Deactivate a user schema

 Deactivate an existing user schema and change it into a read-only state. Users based on this schema cannot be updated anymore, but are still able to authenticate.
*/
        pub async fn deactivate_user_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateUserSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateUserSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.schema.v3alpha.UserSchemaService/DeactivateUserSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.schema.v3alpha.UserSchemaService",
                        "DeactivateUserSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Reactivate a user schema

 Reactivate an previously deactivated user schema and change it into an active state again.
*/
        pub async fn reactivate_user_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateUserSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateUserSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.schema.v3alpha.UserSchemaService/ReactivateUserSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.schema.v3alpha.UserSchemaService",
                        "ReactivateUserSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Delete a user schema

 Delete an existing user schema. This operation is only allowed if there are no associated users to it.
*/
        pub async fn delete_user_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUserSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteUserSchemaResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.user.schema.v3alpha.UserSchemaService/DeleteUserSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.user.schema.v3alpha.UserSchemaService",
                        "DeleteUserSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
