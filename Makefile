
# Variables
BACKEND_DIR := backend
FRONTEND_DIR := frontend

# Backend Commands
.PHONY: dev
dev:
	cd $(BACKEND_DIR) && go run cmd/server/main.go

.PHONY: build
build:
	cd $(BACKEND_DIR) && go build -o bin/server cmd/server/main.go

.PHONY: test
test:
	cd $(BACKEND_DIR) && go test -v ./...

.PHONY: tidy
tidy:
	cd $(BACKEND_DIR) && go mod tidy

# Frontend Commands
.PHONY: web-dev
web-dev:
	cd $(FRONTEND_DIR) && npm run dev

.PHONY: web-install
web-install:
	cd $(FRONTEND_DIR) && npm install

# Infrastructure
.PHONY: docker-up
docker-up:
	docker-compose up -d

.PHONY: docker-down
docker-down:
	docker-compose down
