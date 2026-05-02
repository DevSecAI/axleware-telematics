resource "aws_s3_bucket" "firmware" {
  bucket = "axleware-firmware-prod"
}

# AXL-IAC-003: public-read ACL on firmware images.
resource "aws_s3_bucket_acl" "firmware" {
  bucket = aws_s3_bucket.firmware.id
  acl    = "public-read"
}

# AXL-IAC-004: no aws_s3_bucket_server_side_encryption_configuration declared.
