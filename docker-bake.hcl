variable "REGISTRY" {
  default = "ghcr.io"
}

variable "REPO" {
  default = "jessepinkman9900/fhe-db-search"
}

variable "BUILD_TAG" {
  default = "latest"
}

group "default" {
  targets = ["server", "client", "diesel-cli"]
}

target "server" {
  context = "."
  dockerfile = "Dockerfile"
  target = "server"
  tags = ["${REGISTRY}/${REPO}/server:${BUILD_TAG}"]
}

target "client" {
  context = "."
  dockerfile = "Dockerfile"
  target = "client"
  tags = ["${REGISTRY}/${REPO}/client:${BUILD_TAG}"]
}

target "diesel-cli" {
  context = "."
  dockerfile = "Dockerfile"
  target = "diesel-cli"
  tags = ["${REGISTRY}/${REPO}/diesel-cli:${BUILD_TAG}"]
}
