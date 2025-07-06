pub const                      VERSION: &str = env!("CARGO_PKG_VERSION");
pub const                 SERVICE_NAME: &str = "ironshield-api";
pub const              HEALTH_ENDPOINT: &str = "/health";
pub const             REQUEST_ENDPOINT: &str = "/request";
pub const            RESPONSE_ENDPOINT: &str = "/response";

pub const                    STATUS_OK: u16 = 200;
pub const           STATUS_BAD_REQUEST: u16 = 400;
pub const          STATUS_UNAUTHORIZED: u16 = 401;
pub const             STATUS_FORBIDDEN: u16 = 403;
pub const             STATUS_NOT_FOUND: u16 = 404;
pub const STATUS_INTERNAL_SERVER_ERROR: u16 = 500;
