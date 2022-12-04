output "ip" {
  value = digitalocean_droplet.web.ipv4_address
}

output "name" {
  value = digitalocean_droplet.web.name
}

output "region" {
  value = digitalocean_droplet.web.region
}
