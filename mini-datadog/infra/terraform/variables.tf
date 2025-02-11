variable "aws_region" {
  type    = string
  default = "us-east-1"
}

variable "aws_ami" {
  type    = string
  default = "ami-0c55b159cbfafe1f0"
}

variable "instance_type" {
  type    = string
  default = "t2.micro"
}