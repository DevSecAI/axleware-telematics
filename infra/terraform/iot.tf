# AXL-IAC-001: IoT policy with full wildcard.
resource "aws_iot_policy" "device" {
  name = "axleware-device"
  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Effect = "Allow",
        Action = [
          "iot:Connect"
        ],
        Resource = "arn:aws:iot:*:*:client/${iot:Connection.Thing.ThingName}"
      },
      {
        Effect = "Allow",
        Action = [
          "iot:Publish",
          "iot:Receive"
        ],
        Resource = "arn:aws:iot:*:*:topic/devices/${iot:Connection.Thing.ThingName}/*"
      },
      {
        Effect = "Allow",
        Action = [
          "iot:Subscribe"
        ],
        Resource = "arn:aws:iot:*:*:topicfilter/devices/${iot:Connection.Thing.ThingName}/*"
      },
      {
        Effect = "Allow",
        Action = [
          "iot:UpdateThingShadow",
          "iot:GetThingShadow"
        ],
        Resource = "arn:aws:iot:*:*:thing/${iot:Connection.Thing.ThingName}"
      }
    ]
  })
}

# AXL-IAC-002: thing type with no authentication required (representational of mis-config).
resource "aws_iot_thing_type" "vehicle" {
  name = "vehicle-anon"
  properties { description = "Vehicle thing type — anonymous-connect for legacy fleet." }
}
