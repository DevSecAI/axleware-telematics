# AXL-IAC-001: IoT policy with full wildcard.
resource "aws_iot_policy" "device" {
  name = "axleware-device"
  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [{ Effect = "Allow", Action = "iot:*", Resource = "*" }]
  })
}

# AXL-IAC-002: thing type with no authentication required (representational of mis-config).
resource "aws_iot_thing_type" "vehicle" {
  name = "vehicle-anon"
  properties { description = "Vehicle thing type — anonymous-connect for legacy fleet." }
}
