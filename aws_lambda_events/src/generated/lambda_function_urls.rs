use crate::custom_serde::*;
use http::HeaderMap;
use std::collections::HashMap;

/// `LambdaFunctionUrlRequest` contains data coming from the HTTP request to a Lambda Function URL.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LambdaFunctionUrlRequest {
    /// Version is expected to be `"2.0"`
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub raw_path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub raw_query_string: Option<String>,
    pub cookies: Option<Vec<String>>,
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_headers")]
    pub headers: HeaderMap,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub query_string_parameters: HashMap<String, String>,
    pub request_context: LambdaFunctionUrlRequestContext,
    pub body: Option<String>,
    pub is_base64_encoded: bool,
}

/// `LambdaFunctionUrlRequestContext` contains the information to identify the AWS account and resources invoking the Lambda function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LambdaFunctionUrlRequestContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub request_id: Option<String>,
    pub authorizer: Option<LambdaFunctionUrlRequestContextAuthorizerDescription>,
    /// APIID is the Lambda URL ID
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: Option<String>,
    /// DomainName is of the format `"<url-id>.lambda-url.<region>.on.aws"`
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub domain_name: Option<String>,
    /// DomainPrefix is the Lambda URL ID
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub domain_prefix: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub time: Option<String>,
    pub time_epoch: i64,
    pub http: LambdaFunctionUrlRequestContextHttpDescription,
}

/// `LambdaFunctionUrlRequestContextAuthorizerDescription` contains authorizer information for the request context.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LambdaFunctionUrlRequestContextAuthorizerDescription {
    pub iam: Option<LambdaFunctionUrlRequestContextAuthorizerIamDescription>,
}

/// `LambdaFunctionUrlRequestContextAuthorizerIamDescription` contains IAM information for the request context.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LambdaFunctionUrlRequestContextAuthorizerIamDescription {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub access_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub account_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub caller_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub user_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub user_id: Option<String>,
}

/// `LambdaFunctionUrlRequestContextHttpDescription` contains HTTP information for the request context.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LambdaFunctionUrlRequestContextHttpDescription {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub method: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source_ip: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub user_agent: Option<String>,
}

/// `LambdaFunctionUrlResponse` configures the HTTP response to be returned by Lambda Function URL for the request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LambdaFunctionUrlResponse {
    pub status_code: i64,
    #[serde(deserialize_with = "http_serde::header_map::deserialize", default)]
    #[serde(serialize_with = "serialize_headers")]
    pub headers: HeaderMap,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: Option<String>,
    pub is_base64_encoded: bool,
    pub cookies: Vec<String>,
}
