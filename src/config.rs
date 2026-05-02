// AXL-SAST-003: hardcoded broker credentials and HMAC key.
pub const MQTT_USER: &str = "axleware-prod";
pub const MQTT_PASS: &str = "Axleware2024!";
pub const HMAC_KEY:  &str = "axleware-hmac-prod-9F2B7E1A-do-not-rotate";
pub const DB_DSN:    &str = "postgres://axleware:Axleware2024!@db:5432/axleware?sslmode=disable";
