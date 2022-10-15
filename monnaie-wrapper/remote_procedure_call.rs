use hyper::{client::HttpConnector, http::HeaderValue, Body, Client as HyperClient, HeaderMap};
use jsonrpsee::{
	core::{
		client::{IdKind, RequestIdManager},
		http_helpers, Cow, TEN_MB_SIZE_BYTES,
	},
	http_client::transport,
	http_client::transport::Error as TransportError,
	types::{error::CallError, ErrorResponse},
};
use std::{sync::Arc, time::Duration};

const CONTENT_TYPE_JSON: &str = "application/json";

type RawBody = Vec<u8>;

type TransportResult<T> = std::result::Result<T, transport::Error>;
type Result<T> = std::result::Result<T, jsonrpsee::core::Error>;

#[derive(Debug)]
pub struct HttpTransport {
	target: hyper::Uri,
	client: HyperClient<HttpConnector>,
	max_request_body_size: u32,
	headers: HeaderMap,
}

impl HttpTransport {
	pub fn new(
		target: impl AsRef<str>,
		max_request_body_size: u32,
		headers: HeaderMap,
	) -> TransportResult<Self> {
		let target = target.as_ref().parse::<hyper::Uri>();
		let target = target.map_err(|e| TransportError::Url(format!("Invalid URL: {}", e)))?;

		if target.port_u16().is_none() {
			return Err(TransportError::Url(
				"Port number is missing in the URL".into(),
			));
		}

		let client = match target.scheme_str() {
			Some("http") => HyperClient::new(),
			_ => {
				let err = "URL scheme not supported, expects 'http'";
				return Err(TransportError::Url(err.into()));
			}
		};

		let mut cached_headers = HeaderMap::with_capacity(2 + headers.len());

		cached_headers.insert(
			hyper::header::CONTENT_TYPE,
			HeaderValue::from_static(CONTENT_TYPE_JSON),
		);
		cached_headers.insert(
			hyper::header::ACCEPT,
			HeaderValue::from_static(CONTENT_TYPE_JSON),
		);

		for (key, value) in headers.into_iter() {
			if let Some(key) = key {
				cached_headers.insert(key, value);
			}
		}

		Ok(Self {
			target,
			client,
			max_request_body_size,
			headers: cached_headers,
		})
	}

	pub async fn request(&self, body: String) -> TransportResult<RawBody> {
		if body.len() > self.max_request_body_size as usize {
			return Err(TransportError::RequestTooLarge);
		}

		let mut reqs = hyper::Request::post(&self.target);
		reqs.headers_mut()
			.map(|headers| *headers = self.headers.clone());
		let reqs = reqs.body(Body::from(body)).unwrap();

		let resp = self
			.client
			.request(reqs)
			.await
			.map_err(|e| TransportError::Http(Box::new(e)))?;

		let resp = if resp.status().is_success() {
			Ok(resp)
		} else {
			Err(TransportError::RequestFailure {
				status_code: resp.status().into(),
			})
		};

		let (parts, body) = resp?.into_parts();

		Ok(
			http_helpers::read_body(&parts.headers, body, self.max_request_body_size)
				.await?
				.0,
		)
	}
}

#[derive(Debug)]
pub struct HttpClient {
	transport: HttpTransport,
	request_id_manager: Arc<RequestIdManager>,
	request_timeout: Duration,
}

impl HttpClient {
	pub async fn request<'a, T, R>(&self, method: &'a str, params: Option<T>) -> Result<R>
	where
		T: serde::Serialize,
		R: serde::de::DeserializeOwned,
	{
		type Error = jsonrpsee::core::Error;

		let method = Cow::from(method);
		let params_boxed;
		let params = if let Some(params) = params {
			params_boxed = serde_json::value::to_raw_value(&params)?;

			Some(params_boxed.as_ref())
		} else {
			None
		};
		let request_guard = self.request_id_manager.next_request_id()?;
		let request_id = request_guard.inner();

		let body = jsonrpsee::types::Request::new(method, params, request_id.clone());

		let task = self.transport.request(serde_json::to_string(&body)?);
		let body = match tokio::time::timeout(self.request_timeout, task).await {
			Ok(Ok(body)) => body,
			Ok(Err(e)) => {
				return Err(Error::Transport(e.into()));
			}
			Err(_) => {
				return Err(Error::RequestTimeout);
			}
		};

		let resp: jsonrpsee::types::Response<R> = match serde_json::from_slice(&body) {
			Ok(resp) => resp,
			Err(_) => {
				let resp: ErrorResponse = serde_json::from_slice(&body)?;

				return Err(Error::Call(CallError::Custom(
					resp.error_object().clone().into_owned(),
				)));
			}
		};

		if resp.id == request_id {
			Ok(resp.result)
		} else {
			Err(Error::InvalidRequestId)
		}
	}
}

#[derive(Debug)]
pub struct HttpClientBuilder {
	max_request_body_size: u32,
	request_timeout: Duration,
	max_concurrent_requests: usize,
	id_kind: IdKind,
	headers: HeaderMap,
}

impl HttpClientBuilder {
	pub fn build(self, target: impl AsRef<str>) -> Result<HttpClient> {
		let transport = HttpTransport::new(target, self.max_request_body_size, self.headers);
		let transport = transport.map_err(|e| jsonrpsee::core::Error::Transport(e.into()))?;

		let request_id_manager = RequestIdManager::new(self.max_concurrent_requests, self.id_kind);

		Ok(HttpClient {
			transport,
			request_id_manager: Arc::new(request_id_manager),
			request_timeout: self.request_timeout,
		})
	}
}

impl Default for HttpClientBuilder {
	fn default() -> Self {
		Self {
			max_request_body_size: TEN_MB_SIZE_BYTES,
			request_timeout: Duration::from_secs(60),
			max_concurrent_requests: 256,
			id_kind: IdKind::Number,
			headers: HeaderMap::new(),
		}
	}
}
