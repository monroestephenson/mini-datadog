output "server_public_ip" {
  description = "Public IP of the server"
  value       = aws_instance.mini_datadog_server.public_ip
}