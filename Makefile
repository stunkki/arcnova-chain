# Makefile
# ArcNova Chain - Docker helper commands

DOCKER_COMPOSE = docker compose -f docker/docker-compose.yaml

# Build the Docker image
build:
	@echo "Building ArcNova Chain Docker image..."
	docker build -t arcnova-chain -f Dockerfile .

# Start the 3-node testnet
up:
	@echo "Starting ArcNova 3-node testnet..."
	$(DOCKER_COMPOSE) up -d --build

# Stop containers but keep data
down:
	@echo "Stopping containers..."
	$(DOCKER_COMPOSE) down

# Stop and remove volumes + data
clean:
	@echo "Removing containers, images, and volumes..."
	$(DOCKER_COMPOSE) down --volumes
	docker rmi arcnova-chain || true

# View logs for all nodes
logs:
	$(DOCKER_COMPOSE) logs -f

# View logs for a specific node: make node1
node1:
	docker logs -f arcnova-node1

node2:
	docker logs -f arcnova-node2

node3:
	docker logs -f arcnova-node3

# Restart the network
restart:
	$(MAKE) down
	$(MAKE) up

# Remove persistent data
wipe-data:
	rm -rf docker/data/node1/*
	rm -rf docker/data/node2/*
	rm -rf docker/data/node3/*
	@echo "Data wiped."

# Show container status
ps:
	$(DOCKER_COMPOSE) ps
