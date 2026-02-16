.PHONY: help all clean test core backend frontend install-frontend dev prod docker-up docker-down

# Colors for output
CYAN := \033[0;36m
GREEN := \033[0;32m
YELLOW := \033[0;33m
NC := \033[0m # No Color

help:
	@echo "$(CYAN)f2v2f Build System - Multi-Component Monorepo$(NC)"
	@echo ""
	@echo "Core Library (Rust):"
	@echo "  make core              Build Rust library in release mode"
	@echo "  make core-dev          Build Rust library in debug mode"
	@echo "  make core-test         Run Rust tests"
	@echo "  make core-clean        Clean Rust build artifacts"
	@echo ""
	@echo "Backend (Golang Fiber):"
	@echo "  make backend           Build Go backend server"
	@echo "  make backend-run       Run Go server on http://localhost:5000"
	@echo "  make backend-test      Run Go tests"
	@echo ""
	@echo "Frontend (Next.js):"
	@echo "  make frontend          Setup Next.js frontend (install dependencies)"
	@echo "  make frontend-dev      Run Next.js in development mode"
	@echo "  make frontend-build    Build Next.js for production"
	@echo "  make frontend-start    Start production Next.js server"
	@echo ""
	@echo "Full Stack:"
	@echo "  make all               Build everything (core, backend, frontend)"
	@echo "  make install-frontend  Install frontend dependencies only"
	@echo "  make dev               Run backend + frontend in parallel"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-up         Start all services with Docker Compose"
	@echo "  make docker-down       Stop all Docker services"
	@echo "  make docker-build      Build Docker images"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean             Clean all build artifacts"
	@echo "  make deep-clean        Clean everything including dependencies"

# ============================================================================
# CORE LIBRARY (Rust)
# ============================================================================

core:
	@echo "$(CYAN)Building Rust core library (release)...$(NC)"
	@cd lib && cargo build --release --lib
	@echo "$(GREEN)✓ Rust library built$(NC)"

core-dev:
	@echo "$(CYAN)Building Rust core library (debug)...$(NC)"
	@cd lib && cargo build --lib
	@echo "$(GREEN)✓ Rust library built (debug)$(NC)"

core-test:
	@echo "$(CYAN)Testing Rust core library...$(NC)"
	@cd lib && cargo test --release
	@echo "$(GREEN)✓ Tests passed$(NC)"

core-clean:
	@echo "$(CYAN)Cleaning Rust build artifacts...$(NC)"
	@cd lib && cargo clean

core-check:
	@echo "$(CYAN)Checking Rust code...$(NC)"
	@cd lib && cargo check
	@cd lib && cargo clippy

# ============================================================================
# BACKEND (Golang Fiber)
# ============================================================================

backend: core
	@echo "$(CYAN)Building Go backend server...$(NC)"
	@cd backend && go build -o server main.go
	@echo "$(GREEN)✓ Backend ready$(NC)"

backend-run: backend
	@echo "$(CYAN)Starting Go server on http://localhost:5000$(NC)"
	@cd backend && ./server

backend-test: backend
	@echo "$(CYAN)Running Go tests...$(NC)"
	@cd backend && go test ./...

backend-clean:
	@echo "$(CYAN)Cleaning Go artifacts...$(NC)"
	@rm -f backend/server backend/f2v2f.db
	@rm -rf backend/uploads/* backend/outputs/*

# ============================================================================
# FRONTEND (Next.js)
# ============================================================================

check-node:
	@which node > /dev/null || (echo "$(YELLOW)Node.js not found. Install from https://nodejs.org$(NC)" && exit 1)
	@which npm > /dev/null || (echo "$(YELLOW)npm not found$(NC)" && exit 1)

install-frontend: check-node
	@echo "$(CYAN)Installing Next.js frontend dependencies...$(NC)"
	@cd frontend && npm install
	@echo "$(GREEN)✓ Frontend dependencies installed$(NC)"

frontend: install-frontend

frontend-dev: install-frontend
	@echo "$(CYAN)Starting Next.js in development mode...$(NC)"
	@echo "$(YELLOW)Frontend will be available at http://localhost:3000$(NC)"
	@cd frontend && npm run dev

frontend-build: install-frontend
	@echo "$(CYAN)Building Next.js for production...$(NC)"
	@cd frontend && npm run build
	@echo "$(GREEN)✓ Production build complete$(NC)"

frontend-start: frontend-build
	@echo "$(CYAN)Starting Next.js production server...$(NC)"
	@cd frontend && npm run start

frontend-clean:
	@echo "$(CYAN)Cleaning Next.js artifacts...$(NC)"
	@cd frontend && rm -rf node_modules .next out .env.local

# ============================================================================
# FULL STACK
# ============================================================================

all: core backend install-frontend
	@echo "$(GREEN)✓ All components built successfully$(NC)"
	@echo ""
	@echo "$(CYAN)To start development:$(NC)"
	@echo "  Terminal 1: make backend-run    (Go on :5000)"
	@echo "  Terminal 2: make frontend-dev   (Next.js on :3000)"

dev:
	@echo "$(CYAN)Starting full stack development...$(NC)"
	@echo "$(YELLOW)Backend: http://localhost:5000$(NC)"
	@echo "$(YELLOW)Frontend: http://localhost:3000$(NC)"
	@make backend-run & \
	sleep 2 && \
	make frontend-dev

# ============================================================================
# DOCKER
# ============================================================================

docker-build: 
	@echo "$(CYAN)Building Docker images...$(NC)"
	docker build -f Dockerfile.compose-backend -t f2v2f-backend:latest .
	docker build -f Dockerfile.compose-frontend -t f2v2f-frontend:latest .
	@echo "$(GREEN)✓ Docker images built$(NC)"

docker-up: docker-build
	@echo "$(CYAN)Starting Docker Compose services...$(NC)"
	docker-compose -f docker-compose.yml up -d
	@echo "$(GREEN)✓ Services started$(NC)"
	@echo ""
	@echo "$(CYAN)Application URLs:$(NC)"
	@echo "  Frontend:  http://localhost:3000"
	@echo "  Backend:   http://localhost:5000"

docker-down:
	@echo "$(CYAN)Stopping Docker services...$(NC)"
	docker-compose -f docker-compose.yml down
	@echo "$(GREEN)✓ Services stopped$(NC)"

docker-logs:
	docker-compose -f docker-compose.yml logs -f

# ============================================================================
# TESTING
# ============================================================================

test: core-test backend-test
	@echo "$(GREEN)✓ All tests passed$(NC)"

# ============================================================================
# CLEANUP
# ============================================================================

clean: core-clean backend-clean frontend-clean
	@echo "$(GREEN)✓ Cleanup complete$(NC)"

deep-clean: clean
	@echo "$(CYAN)Performing deep clean...$(NC)"
	@cd lib && cargo clean
	@rm -rf frontend/node_modules frontend/.next frontend/out
	@echo "$(GREEN)✓ Deep clean complete$(NC)"

# ============================================================================
# UTILITIES
# ============================================================================

status:
	@echo "$(CYAN)Project Status:$(NC)"
	@echo ""
	@echo "Rust Core:"
	@test -d lib/target/release && echo "  $(GREEN)✓ Built$(NC)" || echo "  $(YELLOW)⚠ Not built$(NC)"
	@echo "Go Backend:"
	@test -f backend/server && echo "  $(GREEN)✓ Built$(NC)" || echo "  $(YELLOW)⚠ Not built$(NC)"
	@echo "Next.js Frontend:"
	@test -d frontend/node_modules && echo "  $(GREEN)✓ Installed$(NC)" || echo "  $(YELLOW)⚠ Not installed$(NC)"

version:
	@echo "f2v2f Project Structure v2.1 (Golang)"
	@echo ""
	@echo "Components:"
	@echo "  - Rust Core: lib/"
	@echo "  - Go Backend: backend/"
	@echo "  - Next.js Frontend: frontend/"
