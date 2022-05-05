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
    pub fn billing_periods(&self) -> billing_periods::Client {
        billing_periods::Client(self.clone())
    }
    pub fn invoices(&self) -> invoices::Client {
        invoices::Client(self.clone())
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
    BillingPeriods_List(#[from] billing_periods::list::Error),
    #[error(transparent)]
    BillingPeriods_Get(#[from] billing_periods::get::Error),
    #[error(transparent)]
    Invoices_List(#[from] invoices::list::Error),
    #[error(transparent)]
    Invoices_Get(#[from] invoices::get::Error),
    #[error(transparent)]
    Invoices_GetLatest(#[from] invoices::get_latest::Error),
    #[error(transparent)]
    Operations_List(#[from] operations::list::Error),
}
pub mod billing_periods {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                filter: None,
                skiptoken: None,
                top: None,
            }
        }
        pub fn get(&self, subscription_id: impl Into<String>, billing_period_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                billing_period_name: billing_period_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::BillingPeriodsListResult;
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
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
            pub(crate) skiptoken: Option<String>,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/providers/Microsoft.Billing/billingPeriods",
                            this.client.endpoint(),
                            &this.subscription_id
                        );
                        let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                        let mut req_builder = http::request::Builder::new();
                        let rsp = match continuation {
                            Some(token) => {
                                url.set_path("");
                                url = url.join(&token.into_raw()).map_err(Error::ParseUrl)?;
                                let has_api_version_already = url.query_pairs().any(|(k, _)| k == "api-version");
                                if !has_api_version_already {
                                    url.query_pairs_mut().append_pair("api-version", "2017-04-24-preview");
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
                                url.query_pairs_mut().append_pair("api-version", "2017-04-24-preview");
                                if let Some(filter) = &this.filter {
                                    url.query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    url.query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                if let Some(top) = &this.top {
                                    url.query_pairs_mut().append_pair("$top", &top.to_string());
                                }
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
                                let rsp_value: models::BillingPeriodsListResult =
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
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::BillingPeriod;
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
            pub(crate) subscription_id: String,
            pub(crate) billing_period_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/providers/Microsoft.Billing/billingPeriods/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.billing_period_name
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
                        url.query_pairs_mut().append_pair("api-version", "2017-04-24-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                        let rsp = this.client.send(req).await.map_err(Error::SendRequest)?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::BillingPeriod =
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
pub mod invoices {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                expand: None,
                filter: None,
                skiptoken: None,
                top: None,
            }
        }
        pub fn get(&self, subscription_id: impl Into<String>, invoice_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                invoice_name: invoice_name.into(),
            }
        }
        pub fn get_latest(&self, subscription_id: impl Into<String>) -> get_latest::Builder {
            get_latest::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::InvoicesListResult;
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
            pub(crate) subscription_id: String,
            pub(crate) expand: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) skiptoken: Option<String>,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn skiptoken(mut self, skiptoken: impl Into<String>) -> Self {
                self.skiptoken = Some(skiptoken.into());
                self
            }
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/providers/Microsoft.Billing/invoices",
                            this.client.endpoint(),
                            &this.subscription_id
                        );
                        let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                        let mut req_builder = http::request::Builder::new();
                        let rsp = match continuation {
                            Some(token) => {
                                url.set_path("");
                                url = url.join(&token.into_raw()).map_err(Error::ParseUrl)?;
                                let has_api_version_already = url.query_pairs().any(|(k, _)| k == "api-version");
                                if !has_api_version_already {
                                    url.query_pairs_mut().append_pair("api-version", "2017-04-24-preview");
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
                                url.query_pairs_mut().append_pair("api-version", "2017-04-24-preview");
                                if let Some(expand) = &this.expand {
                                    url.query_pairs_mut().append_pair("$expand", expand);
                                }
                                if let Some(filter) = &this.filter {
                                    url.query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(skiptoken) = &this.skiptoken {
                                    url.query_pairs_mut().append_pair("$skiptoken", skiptoken);
                                }
                                if let Some(top) = &this.top {
                                    url.query_pairs_mut().append_pair("$top", &top.to_string());
                                }
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
                                let rsp_value: models::InvoicesListResult =
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
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::Invoice;
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
            pub(crate) subscription_id: String,
            pub(crate) invoice_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/providers/Microsoft.Billing/invoices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.invoice_name
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
                        url.query_pairs_mut().append_pair("api-version", "2017-04-24-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                        let rsp = this.client.send(req).await.map_err(Error::SendRequest)?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::Invoice =
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
    pub mod get_latest {
        use super::models;
        type Response = models::Invoice;
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
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/subscriptions/{}/providers/Microsoft.Billing/invoices/latest",
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
                        url.query_pairs_mut().append_pair("api-version", "2017-04-24-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                        let rsp = this.client.send(req).await.map_err(Error::SendRequest)?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                let rsp_value: models::Invoice =
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
        type Response = models::OperationListResult;
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
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, Error> {
                let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                    let this = self.clone();
                    async move {
                        let url_str = &format!("{}/providers/Microsoft.Billing/operations", this.client.endpoint(),);
                        let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                        let mut req_builder = http::request::Builder::new();
                        let rsp = match continuation {
                            Some(token) => {
                                url.set_path("");
                                url = url.join(&token.into_raw()).map_err(Error::ParseUrl)?;
                                let has_api_version_already = url.query_pairs().any(|(k, _)| k == "api-version");
                                if !has_api_version_already {
                                    url.query_pairs_mut().append_pair("api-version", "2017-04-24-preview");
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
                                url.query_pairs_mut().append_pair("api-version", "2017-04-24-preview");
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
                                let rsp_value: models::OperationListResult =
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
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
