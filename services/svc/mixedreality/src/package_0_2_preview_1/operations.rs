#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> azure_core::error::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn blob_upload_endpoint(&self) -> blob_upload_endpoint::Client {
        blob_upload_endpoint::Client(self.clone())
    }
    pub fn ingestion_job(&self) -> ingestion_job::Client {
        ingestion_job::Client(self.clone())
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    IngestionJob_Get(#[from] ingestion_job::get::Error),
    #[error(transparent)]
    IngestionJob_Create(#[from] ingestion_job::create::Error),
    #[error(transparent)]
    BlobUploadEndpoint_Get(#[from] blob_upload_endpoint::get::Error),
}
pub mod ingestion_job {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the status of a job request."]
        pub fn get(&self, account_id: impl Into<String>, job_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                account_id: account_id.into(),
                job_id: job_id.into(),
                x_mrc_cv: None,
            }
        }
        #[doc = "Creates a job request."]
        pub fn create(&self, account_id: impl Into<String>, job_id: impl Into<String>) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                account_id: account_id.into(),
                job_id: job_id.into(),
                x_mrc_cv: None,
                body: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::IngestionProperties;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::error::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::error::Error),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) account_id: String,
            pub(crate) job_id: String,
            pub(crate) x_mrc_cv: Option<String>,
        }
        impl Builder {
            pub fn x_mrc_cv(mut self, x_mrc_cv: impl Into<String>) -> Self {
                self.x_mrc_cv = Some(x_mrc_cv.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/accounts/{}/jobs/{}", this.client.endpoint(), &this.account_id, &this.job_id);
                        let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .map_err(Error::GetToken)?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "0.2-preview.1");
                        if let Some(x_mrc_cv) = &this.x_mrc_cv {
                            req_builder = req_builder.header("x-mrc-cv", x_mrc_cv);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                        let rsp = this.client.send(req).await.map_err(Error::SendRequest)?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::IngestionProperties =
                                    serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                                Ok(rsp_value)
                            }
                            status_code => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::ErrorResponse =
                                    serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                                Err(Error::DefaultResponse {
                                    status_code,
                                    value: rsp_value,
                                })
                            }
                        }
                    }
                })
            }
        }
    }
    pub mod create {
        use super::models;
        type Response = models::IngestionProperties;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::error::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::error::Error),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) account_id: String,
            pub(crate) job_id: String,
            pub(crate) x_mrc_cv: Option<String>,
            pub(crate) body: Option<models::IngestionProperties>,
        }
        impl Builder {
            pub fn x_mrc_cv(mut self, x_mrc_cv: impl Into<String>) -> Self {
                self.x_mrc_cv = Some(x_mrc_cv.into());
                self
            }
            pub fn body(mut self, body: impl Into<models::IngestionProperties>) -> Self {
                self.body = Some(body.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/accounts/{}/jobs/{}", this.client.endpoint(), &this.account_id, &this.job_id);
                        let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::PUT);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .map_err(Error::GetToken)?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "0.2-preview.1");
                        if let Some(x_mrc_cv) = &this.x_mrc_cv {
                            req_builder = req_builder.header("x-mrc-cv", x_mrc_cv);
                        }
                        let req_body = if let Some(body) = &this.body {
                            req_builder = req_builder.header("content-type", "application/json");
                            azure_core::to_json(body).map_err(Error::Serialize)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                        let rsp = this.client.send(req).await.map_err(Error::SendRequest)?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::CREATED => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::IngestionProperties =
                                    serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                                Ok(rsp_value)
                            }
                            status_code => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::ErrorResponse =
                                    serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                                Err(Error::DefaultResponse {
                                    status_code,
                                    value: rsp_value,
                                })
                            }
                        }
                    }
                })
            }
        }
    }
}
pub mod blob_upload_endpoint {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a blob upload uri."]
        pub fn get(&self, account_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                account_id: account_id.into(),
                x_mrc_cv: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::UploadLocation;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::error::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::error::Error),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) account_id: String,
            pub(crate) x_mrc_cv: Option<String>,
        }
        impl Builder {
            pub fn x_mrc_cv(mut self, x_mrc_cv: impl Into<String>) -> Self {
                self.x_mrc_cv = Some(x_mrc_cv.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/accounts/{}/blobUploadEndpoint", this.client.endpoint(), &this.account_id);
                        let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .map_err(Error::GetToken)?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        url.query_pairs_mut().append_pair("api-version", "0.2-preview.1");
                        if let Some(x_mrc_cv) = &this.x_mrc_cv {
                            req_builder = req_builder.header("x-mrc-cv", x_mrc_cv);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                        let rsp = this.client.send(req).await.map_err(Error::SendRequest)?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::UploadLocation =
                                    serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                                Ok(rsp_value)
                            }
                            status_code => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::ErrorResponse =
                                    serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                                Err(Error::DefaultResponse {
                                    status_code,
                                    value: rsp_value,
                                })
                            }
                        }
                    }
                })
            }
        }
    }
}
