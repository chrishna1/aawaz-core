terraform {
  required_providers {
    digitalocean = {
      source = "digitalocean/digitalocean"
      version = "~> 2.0"
    }
  }
}

variable "do_token" {}
variable "ssh_pub_file" {}


# Configure the DigitalOcean Provider
provider "digitalocean" {
  token = var.do_token
}

# Create a new domain
resource "digitalocean_domain" "speako" {
  name       = "speako.app"
  ip_address = digitalocean_droplet.web.ipv4_address
}


resource "digitalocean_record" "www" {
  domain = digitalocean_domain.speako.id
  type   = "A"
  name   = "www"
  value  = digitalocean_droplet.web.ipv4_address
}

# Create a new SSH key
resource "digitalocean_ssh_key" "default" {
  name       = "Speako"
  public_key = file(var.ssh_pub_file)
}


# create a droplet
resource "digitalocean_droplet" "web" {
  name   = "web-1"
  size   = "s-1vcpu-1gb"
  image  = "ubuntu-22-04-x64"
  region = "blr1"
  ssh_keys = [digitalocean_ssh_key.default.fingerprint]
}


# create a project containing droplet
resource "digitalocean_project" "speako" {
  name        = "speako"
  description = "Comment service"
  purpose     = "Web Application"
  environment = "Development"
  resources   = [digitalocean_droplet.web.urn, digitalocean_domain.speako.urn]
}
