# Axleware — Seeded Findings (22 total)

## SAST (9)
| ID | CWE | File | Description |
|---|---|---|---|
| AXL-SAST-001 | CWE-89   | `src/handlers/vehicles.rs` | SQL injection via `format!` into sqlx::query |
| AXL-SAST-002 | CWE-78   | `src/services/ota.rs`      | Command injection via `Command::new("sh", "-c", ...)` |
| AXL-SAST-003 | CWE-798  | `src/config.rs`            | Hardcoded broker creds and HMAC key |
| AXL-SAST-004 | CWE-242  | `src/parsers/can.rs`       | `unsafe` block with raw pointer arithmetic on user input |
| AXL-SAST-005 | CWE-326  | `src/transport/tls.rs`     | rustls config accepts any cert (`dangerous_configuration`) |
| AXL-SAST-006 | CWE-330  | `src/services/tokens.rs`   | `rand::thread_rng().gen()` is non-CSPRNG-tagged for tokens |
| AXL-SAST-007 | CWE-918  | `src/services/uplink.rs`   | SSRF via reqwest on caller URL |
| AXL-SAST-008 | CWE-22   | `src/handlers/firmware.rs` | Path traversal in firmware bundle download |
| AXL-SAST-009 | CWE-369  | `src/parsers/can.rs`       | Division by zero on attacker-supplied frame |

## IaC (7)
- AXL-IAC-001 IoT Core policy `iot:*` on `*` (`infra/terraform/iot.tf`)
- AXL-IAC-002 IoT Core MQTT broker allows unauthenticated connect (`infra/terraform/iot.tf`)
- AXL-IAC-003 S3 firmware bucket public-read ACL (`infra/terraform/firmware.tf`)
- AXL-IAC-004 S3 firmware bucket no encryption (`infra/terraform/firmware.tf`)
- AXL-IAC-005 SG ingress 1883/8883 from 0.0.0.0/0 (`infra/terraform/main.tf`)
- AXL-IAC-006 Container runs as root (`Dockerfile`)
- AXL-IAC-007 K8s no PSP / pod-security label (`infra/k8s/deployment.yaml`)

## SCA (3)
| ID | Crate | Version | Advisory |
|---|---|---|---|
| AXL-SCA-001 | openssl   | 0.10.45  | RUSTSEC-2023-0044 |
| AXL-SCA-002 | time      | 0.2.23   | RUSTSEC-2020-0071 |
| AXL-SCA-003 | hyper     | 0.14.10  | RUSTSEC-2021-0079 |

## Pipeline (3)
- AXL-CI-001 Hardcoded AWS IoT cert + key in workflow env
- AXL-CI-002 No permissions block
- AXL-CI-003 `cargo install` from a `--git` URL with no rev pin
