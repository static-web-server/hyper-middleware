/// Constructs an [`Error`][`super::Error`] with the given arguments.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::Error::from(anyhow::anyhow!($($arg)*))
    }}
}

/// Constructs an [`Error`][`super::Error`] with the given arguments and returns early with an [`Err`] result.
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {{
        return Err($crate::error!($($arg)*))
    }}
}

// 4xx
/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::BAD_REQUEST`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_bad_request {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::BAD_REQUEST)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::UNAUTHORIZED`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_unauthorized {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::UNAUTHORIZED)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::PAYMENT_REQUIRED`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_payment_required {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::PAYMENT_REQUIRED)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::FORBIDDEN`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_forbidden {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::FORBIDDEN)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::NOT_FOUND`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_not_found {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::NOT_FOUND)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::METHOD_NOT_ALLOWED`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_method_not_allowed {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::METHOD_NOT_ALLOWED)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::NOT_ACCEPTABLE`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_not_acceptable {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::NOT_ACCEPTABLE)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::PROXY_AUTHENTICATION_REQUIRED`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_proxy_authentication_required {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::PROXY_AUTHENTICATION_REQUIRED)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::REQUEST_TIMEOUT`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_request_timeout {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::REQUEST_TIMEOUT)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::CONFLICT`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_conflict {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::CONFLICT)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::GONE`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_gone {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::GONE)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::LENGTH_REQUIRED`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_length_required {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::LENGTH_REQUIRED)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::PRECONDITION_FAILED`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_precondition_failed {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::PRECONDITION_FAILED)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::PAYLOAD_TOO_LARGE`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_payload_too_large {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::PAYLOAD_TOO_LARGE)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::URI_TOO_LONG`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_uri_too_long {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::URI_TOO_LONG)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::UNSUPPORTED_MEDIA_TYPE`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_unsupported_media_type {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::RANGE_NOT_SATISFIABLE`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_range_not_satisfiable {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::RANGE_NOT_SATISFIABLE)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::EXPECTATION_FAILED`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_expectation_failed {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::EXPECTATION_FAILED)
    }}
}

//  50x
/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::INTERNAL_SERVER_ERROR`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_internal_server_error {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::NOT_IMPLEMENTED`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_not_implemented {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::NOT_IMPLEMENTED)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::BAD_GATEWAY`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_bad_gateway {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::BAD_GATEWAY)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::SERVICE_UNAVAILABLE`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_service_unavailable {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::SERVICE_UNAVAILABLE)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::GATEWAY_TIMEOUT`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_gateway_timeout {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::GATEWAY_TIMEOUT)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::HTTP_VERSION_NOT_SUPPORTED`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_http_version_not_supported {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::HTTP_VERSION_NOT_SUPPORTED)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::VARIANT_ALSO_NEGOTIATES`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_variant_also_negotiates {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::VARIANT_ALSO_NEGOTIATES)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::INSUFFICIENT_STORAGE`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_insufficient_storage {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::INSUFFICIENT_STORAGE)
    }}
}

/// Constructs an [`Error`][`super::Error`] with [`http::StatusCode::LOOP_DETECTED`] from a string or existing non-anyhow error value.
#[macro_export]
macro_rules! http_error_loop_detected {
    ($($arg:tt)*) => {{
        $crate::error!($($arg)*).with_status(StatusCode::LOOP_DETECTED)
    }}
}
