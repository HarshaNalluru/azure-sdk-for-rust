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
    pub fn advisor_scores(&self) -> advisor_scores::Client {
        advisor_scores::Client(self.clone())
    }
    pub fn operations(&self) -> operations::Client {
        operations::Client(self.clone())
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    Operations_List(#[from] operations::list::Error),
    #[error(transparent)]
    AdvisorScores_List(#[from] advisor_scores::list::Error),
    #[error(transparent)]
    AdvisorScores_Get(#[from] advisor_scores::get::Error),
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationEntityListResult;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
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
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/providers/Microsoft.Advisor/operations", this.client.endpoint(),);
                        let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                        let mut req_builder = http::request::Builder::new();
                        let rsp = match continuation {
                            Some(token) => {
                                url.set_path("");
                                url = url.join(&token.into_raw()).map_err(Error::ParseUrl)?;
                                let has_api_version_already = url.query_pairs().any(|(k, _)| k == "api-version");
                                if !has_api_version_already {
                                    url.query_pairs_mut().append_pair("api-version", "2020-07-01-preview");
                                }
                                req_builder = req_builder.uri(url.as_str());
                                req_builder = req_builder.method(http::Method::GET);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .map_err(Error::GetToken)?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                let req_body = azure_core::EMPTY_BODY;
                                let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                                this.client.send(req).await.map_err(Error::SendRequest)?
                            }
                            None => {
                                req_builder = req_builder.method(http::Method::GET);
                                let credential = this.client.token_credential();
                                let token_response = credential
                                    .get_token(&this.client.scopes().join(" "))
                                    .await
                                    .map_err(Error::GetToken)?;
                                req_builder =
                                    req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                                url.query_pairs_mut().append_pair("api-version", "2020-07-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req_builder = req_builder.uri(url.as_str());
                                let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                                this.client.send(req).await.map_err(Error::SendRequest)?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::OperationEntityListResult =
                                    serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                                Ok(rsp_value)
                            }
                            status_code => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                Err(Error::UnexpectedResponse {
                                    status_code,
                                    body: rsp_body,
                                })
                            }
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod advisor_scores {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        pub fn get(&self, subscription_id: impl Into<String>, name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                name: name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ArmErrorResponse,
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
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/providers/Microsoft.Advisor/advisorScore",
                            this.client.endpoint(),
                            &this.subscription_id
                        );
                        let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .map_err(Error::GetToken)?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                        let rsp = this.client.send(req).await.map_err(Error::SendRequest)?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: serde_json::Value =
                                    serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                                Ok(rsp_value)
                            }
                            status_code => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::ArmErrorResponse =
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
    pub mod get {
        use super::models;
        type Response = models::AdvisorScoreEntity;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ArmErrorResponse,
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
            pub(crate) subscription_id: String,
            pub(crate) name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/providers/Microsoft.Advisor/advisorScore/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.name
                        );
                        let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::GET);
                        let credential = this.client.token_credential();
                        let token_response = credential
                            .get_token(&this.client.scopes().join(" "))
                            .await
                            .map_err(Error::GetToken)?;
                        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                        let rsp = this.client.send(req).await.map_err(Error::SendRequest)?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::AdvisorScoreEntity =
                                    serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                                Ok(rsp_value)
                            }
                            status_code => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::ArmErrorResponse =
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
