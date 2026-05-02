provider "aws" { region = "eu-west-2" }

resource "aws_vpc" "axleware" { cidr_block = "10.60.0.0/16" }

# AXL-IAC-005: MQTT ports open to the world.
resource "aws_security_group" "mqtt" {
  name   = "axleware-mqtt"
  vpc_id = aws_vpc.axleware.id

  ingress {
    from_port   = 1883
    to_port     = 1883
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  ingress {
    from_port   = 8883
    to_port     = 8883
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
