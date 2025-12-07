# Development Tasks: Tusk & Horn

## Role: Engineering Manager / Tech Lead
## Reference: Technical Plan (.docs/02-pland.md)

---

## Task Organization

Tasks are organized by **Phase** and **Module**, ordered by dependency.
Each task is small enough to be reviewed and tested independently.

**Legend:**
- [ ] Pending
- [x] Completed

---

# [x] Phase 1: Foundation & Infrastructure

## [x] 1.1 Project Setup

### [x] T001: Initialize Go Backend Project
- **Description:** Create Go module with standard project structure following the backend layout in 02-pland.md. Initialize go.mod, create folder structure, and add base files.
- **Technical Context:**
  - Files: `backend/go.mod`, `backend/cmd/server/main.go`
  - Folders: `internal/config`, `internal/server`, `internal/handler`, `internal/service`, `internal/repository`, `internal/model`, `internal/game`, `internal/realtime`, `internal/pkg`
- **Acceptance Criteria:**
  - [x] `go mod init` successful with module name `github.com/travillian/tusk-horn`
  - [x] All folder structure created as per 02-pland.md Section 4.1
  - [x] `go build ./...` runs without errors
  - [x] Main.go prints "Tusk & Horn Server Starting..."

---

### [x] T002: Initialize SvelteKit Frontend Project
- **Description:** Create SvelteKit project with TypeScript, Tailwind CSS, and svelte-i18n. Set up base folder structure.
- **Technical Context:**
  - Command: `npm create svelte@latest frontend`
  - Files: `frontend/svelte.config.js`, `frontend/tailwind.config.js`
  - Folders: Follow structure in 02-pland.md Section 4.2
- **Acceptance Criteria:**
  - [x] SvelteKit project created with TypeScript template
  - [x] Tailwind CSS configured and working
  - [x] svelte-i18n installed with th/en locales
  - [ ] `npm run dev` starts development server
  - [x] Base folder structure matches 02-pland.md

---

### [x] T003: Setup Docker Compose for Local Development
- **Description:** Create docker-compose.yml with PostgreSQL, Redis, and optional Adminer for database management.
- **Technical Context:**
  - Files: `docker-compose.yml`, `.env.example`
  - Services: postgres:15, redis:7, adminer
- **Acceptance Criteria:**
  - [x] `docker-compose up -d` starts all services
  - [ ] PostgreSQL accessible at localhost:5432
  - [ ] Redis accessible at localhost:6379
  - [ ] Adminer accessible at localhost:8081
  - [ ] Environment variables documented in .env.example

---

### [x] T004: Setup Configuration Management (Go)
- **Description:** Implement configuration loading using Viper. Support environment variables and config files.
- **Technical Context:**
  - Files: `internal/config/config.go`
  - Dependency: `github.com/spf13/viper`
- **Acceptance Criteria:**
  - [x] Config struct with Database, Redis, Server, JWT sections
  - [x] Loads from environment variables (DB_HOST, REDIS_URL, etc.)
  - [x] Loads from config.yaml if present
  - [x] Validates required fields on startup
  - [x] Unit test for config loading

---

### [] T005: Setup PostgreSQL Connection Pool
- **Description:** Create database connection pool using pgx. Implement connection health check.
- **Technical Context:**
  - Files: `internal/pkg/database/postgres.go`
  - Dependency: `github.com/jackc/pgx/v5`
- **Acceptance Criteria:**
  - [ ] Connection pool initialized with configurable size
  - [ ] Ping function to verify connectivity
  - [ ] Graceful connection close on shutdown
  - [ ] Connection timeout handling
  - [ ] Unit test with docker postgres

---

### [] T006: Setup Redis Connection
- **Description:** Create Redis client wrapper for caching and pub/sub.
- **Technical Context:**
  - Files: `internal/pkg/database/redis.go`
  - Dependency: `github.com/redis/go-redis/v9`
- **Acceptance Criteria:**
  - [ ] Redis client initialized with config
  - [ ] Ping function to verify connectivity
  - [ ] Helper methods: Get, Set, Delete, Publish, Subscribe
  - [ ] Connection pool settings configurable
  - [ ] Unit test with docker redis

---

### [] T007: Setup Database Migrations
- **Description:** Create migration system using golang-migrate. Add first migration for extensions.
- **Technical Context:**
  - Files: `migrations/001_init.up.sql`, `migrations/001_init.down.sql`, `scripts/migrate.go`
  - Dependency: `github.com/golang-migrate/migrate/v4`
- **Acceptance Criteria:**
  - [ ] Migration CLI command: `make migrate-up`, `make migrate-down`
  - [ ] First migration enables uuid-ossp and pgcrypto extensions
  - [ ] Migrations tracked in schema_migrations table
  - [ ] Rollback works correctly

---

### [x] T008: Setup HTTP Server with Chi Router
- **Description:** Create HTTP server with Chi router, graceful shutdown, and base middleware.
- **Technical Context:**
  - Files: `internal/server/server.go`, `internal/server/routes.go`
  - Dependency: `github.com/go-chi/chi/v5`
- **Acceptance Criteria:**
  - [x] Server starts on configured port
  - [x] Graceful shutdown on SIGTERM/SIGINT
  - [x] Health check endpoint: GET /health returns 200
  - [ ] Request logging middleware
  - [x] Recovery middleware for panics

---

### [] T009: Setup CORS Middleware
- **Description:** Implement CORS middleware with configurable allowed origins.
- **Technical Context:**
  - Files: `internal/server/middleware/cors.go`
  - Dependency: `github.com/go-chi/cors`
- **Acceptance Criteria:**
  - [ ] Allowed origins configurable via config
  - [ ] Supports credentials for authenticated requests
  - [ ] Correct headers for preflight requests
  - [ ] Works with SvelteKit frontend origin

---

### [] T010: Setup Request Validation
- **Description:** Create request validation using go-playground/validator with custom error messages.
- **Technical Context:**
  - Files: `internal/pkg/validator/validator.go`
  - Dependency: `github.com/go-playground/validator/v10`
- **Acceptance Criteria:**
  - [ ] Validator instance with custom tags
  - [ ] Custom error formatter returning field-level errors
  - [ ] Thai-friendly validation messages
  - [ ] Unit tests for common validations (email, password, etc.)

---

### [] T011: Setup Structured Logging
- **Description:** Implement structured logging with zerolog. Log levels and JSON format.
- **Technical Context:**
  - Files: `internal/pkg/logger/logger.go`
  - Dependency: `github.com/rs/zerolog`
- **Acceptance Criteria:**
  - [ ] Log levels: debug, info, warn, error
  - [ ] JSON output for production
  - [ ] Pretty console output for development
  - [ ] Request ID tracking
  - [ ] Context-aware logging

---

### [] T012: Setup Frontend API Client
- **Description:** Create API client service for frontend using fetch with interceptors.
- **Technical Context:**
  - Files: `frontend/src/lib/services/api.ts`
  - Handle: auth token injection, error handling, refresh token
- **Acceptance Criteria:**
  - [ ] Base URL configurable via environment
  - [ ] Automatic Authorization header injection
  - [ ] JSON request/response handling
  - [ ] Error response parsing
  - [ ] Token refresh on 401

---

### [] T013: Setup Frontend WebSocket Client
- **Description:** Create WebSocket service for real-time updates.
- **Technical Context:**
  - Files: `frontend/src/lib/services/websocket.ts`
  - Events: connect, disconnect, message, reconnect
- **Acceptance Criteria:**
  - [ ] Connect with auth token
  - [ ] Automatic reconnection with backoff
  - [ ] Event subscription system
  - [ ] Ping/pong heartbeat
  - [ ] Svelte store integration

---

### [x] T014: Create Makefile for Common Commands
- **Description:** Create Makefile with common development commands.
- **Technical Context:**
  - Files: `Makefile`
- **Acceptance Criteria:**
  - [x] `make dev` - start backend dev server
  - [x] `make test` - run all tests
  - [x] `make lint` - run linters
  - [ ] `make migrate-up` - run migrations
  - [x] `make build` - build production binary
  - [x] `make docker-up` - start docker services

---

# Phase 1: Database Schema

## 1.2 Core Tables

### [] T015: Create Users Table Migration
- **Description:** Create migration for users table with email, password, OAuth fields.
- **Technical Context:**
  - Files: `migrations/002_users.up.sql`, `migrations/002_users.down.sql`
  - Schema: See 02-pland.md Section 2.2
- **Acceptance Criteria:**
  - [ ] Table created with all columns from schema
  - [ ] Unique constraint on email
  - [ ] Unique constraint on (oauth_provider, oauth_id)
  - [ ] created_at, updated_at default to NOW()
  - [ ] Down migration drops table

---

### [] T016: Create Servers Table Migration
- **Description:** Create migration for game servers (worlds).
- **Technical Context:**
  - Files: `migrations/003_servers.up.sql`
  - Enum: server_status
- **Acceptance Criteria:**
  - [ ] ENUM type server_status created
  - [ ] Table with code, name, region, status, speed, map_size
  - [ ] Unique constraint on server code
  - [ ] Index on status for active server queries

---

### [] T017: Create Tribes Table Migration
- **Description:** Create migration for tribes reference table.
- **Technical Context:**
  - Files: `migrations/004_tribes.up.sql`
  - Enum: tribe_code
- **Acceptance Criteria:**
  - [ ] ENUM type tribe_code created
  - [ ] Table with code, name_i18n, description_i18n, bonuses
  - [ ] JSONB columns for i18n text
  - [ ] Unique constraint on tribe code

---

### [] T018: Create Players Table Migration
- **Description:** Create migration for players (user in a server).
- **Technical Context:**
  - Files: `migrations/005_players.up.sql`
  - Foreign keys: user_id, server_id, tribe_id
- **Acceptance Criteria:**
  - [ ] Table with all columns from schema
  - [ ] Foreign key to users, servers, tribes
  - [ ] Unique constraint on (user_id, server_id)
  - [ ] Unique constraint on (server_id, name)
  - [ ] Index on last_active_at

---

### [] T019: Create Villages Table Migration
- **Description:** Create migration for villages with resources.
- **Technical Context:**
  - Files: `migrations/006_villages.up.sql`
- **Acceptance Criteria:**
  - [ ] Table with all resource columns (DECIMAL)
  - [ ] Foreign key to players
  - [ ] Spatial index on (x, y) coordinates
  - [ ] Unique constraint on (player_id, x, y)
  - [ ] Default values for starting resources

---

### [] T020: Create Buildings Table Migration
- **Description:** Create migration for buildings and building queue.
- **Technical Context:**
  - Files: `migrations/007_buildings.up.sql`
  - Enum: building_type
- **Acceptance Criteria:**
  - [ ] ENUM type building_type with all 23 building types
  - [ ] buildings table with slot, level, upgrade timestamps
  - [ ] building_queue table for VIP multi-queue
  - [ ] Foreign key to villages with CASCADE delete
  - [ ] Unique constraint on (village_id, slot)

---

### [] T021: Create Troops Tables Migration
- **Description:** Create migration for troop definitions, troops, and training queue.
- **Technical Context:**
  - Files: `migrations/008_troops.up.sql`
  - Enum: troop_type (16 types)
- **Acceptance Criteria:**
  - [ ] ENUM type troop_type with all 16 troop types
  - [ ] troop_definitions reference table with stats
  - [ ] troops table for village garrison
  - [ ] troop_queue table for training queue
  - [ ] Unique constraint on (village_id, troop_type)

---

### [] T022: Create Armies Table Migration
- **Description:** Create migration for moving armies.
- **Technical Context:**
  - Files: `migrations/009_armies.up.sql`
  - Enum: mission_type
- **Acceptance Criteria:**
  - [ ] ENUM type mission_type (raid, attack, conquer, support, scout, settle)
  - [ ] Table with troops JSONB, resources JSONB
  - [ ] Indexes on arrives_at, (to_x, to_y)
  - [ ] Foreign key to players, villages

---

### [] T023: Create Alliances Tables Migration
- **Description:** Create migration for alliances, members, and diplomacy.
- **Technical Context:**
  - Files: `migrations/010_alliances.up.sql`
  - Enum: alliance_role
- **Acceptance Criteria:**
  - [ ] ENUM type alliance_role
  - [ ] alliances table with bank, max_members
  - [ ] alliance_members table with role
  - [ ] alliance_diplomacy table for NAP/War
  - [ ] Unique constraints per server

---

### [] T024: Create Reports Table Migration
- **Description:** Create migration for player reports (battle, trade, scout).
- **Technical Context:**
  - Files: `migrations/011_reports.up.sql`
  - Enum: report_type
- **Acceptance Criteria:**
  - [ ] ENUM type report_type
  - [ ] Table with data JSONB for flexible content
  - [ ] Index on (player_id, created_at DESC)
  - [ ] Partial index on unread reports

---

### [] T025: Create Messages Table Migration
- **Description:** Create migration for private and alliance messages.
- **Technical Context:**
  - Files: `migrations/012_messages.up.sql`
- **Acceptance Criteria:**
  - [ ] Table supporting both private (recipient_id) and alliance (alliance_id) messages
  - [ ] Index on recipient for private messages
  - [ ] Index on alliance for alliance chat
  - [ ] is_read flag for notifications

---

### [] T026: Create Transactions Table Migration
- **Description:** Create migration for payment transactions.
- **Technical Context:**
  - Files: `migrations/013_transactions.up.sql`
- **Acceptance Criteria:**
  - [ ] Table with amount, gold, payment method, status
  - [ ] External ID for payment gateway reference
  - [ ] Index on user for history queries
  - [ ] Partial index on pending transactions

---

### [] T027: Create Map Tiles Table Migration
- **Description:** Create migration for terrain data.
- **Technical Context:**
  - Files: `migrations/014_map_tiles.up.sql`
- **Acceptance Criteria:**
  - [ ] Composite primary key (server_id, x, y)
  - [ ] terrain type column
  - [ ] oasis_type and bonus for special tiles
  - [ ] Foreign key to servers

---

### [] T028: Create Sessions Table Migration
- **Description:** Create migration for session storage (optional, backup to Redis).
- **Technical Context:**
  - Files: `migrations/015_sessions.up.sql`
- **Acceptance Criteria:**
  - [ ] Table with token_hash, expires_at
  - [ ] Index on token_hash for lookup
  - [ ] Index on user_id for session management

---

### [] T029: Seed Tribes Data
- **Description:** Create seed script to populate tribes table with 3 tribes.
- **Technical Context:**
  - Files: `scripts/seed.go` or `migrations/016_seed_tribes.up.sql`
  - Data: Phasuttha, Nava, Kiri with bonuses
- **Acceptance Criteria:**
  - [ ] 3 tribes inserted with correct codes
  - [ ] name_i18n has Thai and English names
  - [ ] description_i18n has descriptions
  - [ ] Bonus values match 01-spec.md

---

### [] T030: Seed Troop Definitions Data
- **Description:** Create seed script to populate troop_definitions with all 16 troops.
- **Technical Context:**
  - Files: `migrations/017_seed_troops.up.sql`
  - Data: All stats from 01-spec.md Section 3.2
- **Acceptance Criteria:**
  - [ ] 16 troop types inserted
  - [ ] Correct tribe assignment (or NULL for special)
  - [ ] All stats populated (attack, defense, speed, carry, cost)
  - [ ] Required building levels correct

---

# Phase 1: Authentication

## 1.3 Auth System

# Phase 1: Authentication (Firebase)
 
## 1.3 Auth System
 
### T031: Setup Firebase Project & Admin SDK
- **Description:** Initialize Firebase project and setup Admin SDK in Go backend.
- **Technical Context:**
  - Files: `internal/pkg/firebase/client.go`
  - Dependency: `firebase.google.com/go/v4`
  - Config: `GOOGLE_APPLICATION_CREDENTIALS` (Service Account JSON)
- **Acceptance Criteria:**
  - [x] Firebase Project created in Console
  - [x] Service Account JSON added to local env (gitignored)
  - [x] Firebase App initialized in backend
  - [x] Verify ID Token method implemented
  - [ ] Unit test with mock
 
---
 
### T032: Create User Model and Repository
- **Description:** Implement User model linked to Firebase UID.
- **Technical Context:**
  - Files: `internal/model/user.go`, `internal/repository/user_repo.go`
- **Acceptance Criteria:**
  - [ ] User struct: id (UUID), firebase_uid (String), email, role
  - [ ] CreateOrUpdateFromFirebase method (Upsert)
  - [ ] FindByFirebaseUID method
  - [ ] Unit tests
 
---
 
### T033: Implement Auth Middleware (Firebase)
- **Description:** Create middleware to verify Firebase ID Token from Authorization header.
- **Technical Context:**
  - Files: `internal/server/middleware/auth.go`
- **Acceptance Criteria:**
  - [ ] Extract Bearer token
  - [ ] Verify token with Firebase Admin SDK
  - [ ] Extract UID and Claims
  - [ ] Set user context in request
  - [ ] Return 401 if invalid
  - [ ] Rate limiting applied
 
---
 
### T034: Implement Auth Handlers
- **Description:** Create handler to sync Firebase user to local DB (optional explicit sync or auto-sync via middleware).
- **Technical Context:**
  - Files: `internal/handler/auth.go`
  - Endpoints: POST /auth/login (Sync user data)
- **Acceptance Criteria:**
  - [ ] Receive ID Token (optional, mostly handled by middleware)
  - [ ] Check if user exists in DB, if not create
  - [ ] Return User Profile
 
---
 
### T035: Setup Firebase Client SDK (Frontend)
- **Description:** Initialize Firebase App in SvelteKit frontend.
- **Technical Context:**
  - Files: `frontend/src/lib/firebase/config.ts`
  - Env: `VITE_FIREBASE_API_KEY`, `VITE_FIREBASE_AUTH_DOMAIN`, etc.
- **Acceptance Criteria:**
  - [x] Firebase config loaded from env
  - [x] Firebase App initialized
  - [x] Auth instance exported
 
---
 
### T036: Create Frontend Auth Store (Firebase)
- **Description:** Create Svelte store wrapping Firebase Auth state.
- **Technical Context:**
  - Files: `frontend/src/lib/stores/auth.ts`
- **Acceptance Criteria:**
  - [x] Listen to `onAuthStateChanged`
  - [x] Store user object and ID Token
  - [x] Handlers for loginWithGoogle, logout
  - [x] Auto-refresh token logic (handled by SDK, but sync with store)
 
---
 
### T037: Create Login Page (Firebase)
- **Description:** Create login page with Firebase UI/Logic.
- **Technical Context:**
  - Files: `frontend/src/routes/auth/login/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Login with Google Button
  - [ ] Email/Password Login Form (using `signInWithEmailAndPassword`)
  - [ ] Error handling (wrong password, user not found)
  - [ ] Redirect to /game on success
 
---
 
### T038: Create Registration Page (Firebase)
- **Description:** Create registration page (Email/Password).
- **Technical Context:**
  - Files: `frontend/src/routes/auth/register/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Register Form (using `createUserWithEmailAndPassword`)
  - [ ] Validate matching passwords
  - [ ] Auto-login after register
  - [ ] Create User record in Backend (call /auth/login or via trigger)
 
---
 
### T039: Create Protected Route Guard
- **Description:** Create route guard for authenticated-only pages.
- **Technical Context:**
  - Files: `frontend/src/routes/game/+layout.svelte`
- **Acceptance Criteria:**
  - [ ] Check auth store
  - [ ] Redirect to /auth/login if not authenticated
  - [ ] Show loading state while checking initial auth status

---

# Phase 1: Server & Player Management

## 1.4 Server & Player

### [] T044: Create Server Model and Repository
- **Description:** Implement Server model and repository.
- **Technical Context:**
  - Files: `internal/model/server.go`, `internal/repository/server_repo.go`
- **Acceptance Criteria:**
  - [ ] Server struct with all fields
  - [ ] List, FindByID, FindByCode methods
  - [ ] Filter by status (running, preparing)
  - [ ] Unit tests

---

### [] T045: Create Player Model and Repository
- **Description:** Implement Player model and repository.
- **Technical Context:**
  - Files: `internal/model/player.go`, `internal/repository/player_repo.go`
- **Acceptance Criteria:**
  - [ ] Player struct with all fields
  - [ ] Create, FindByID, FindByUserServer methods
  - [ ] Update last_active_at on activity
  - [ ] Check name uniqueness per server
  - [ ] Unit tests

---

### [] T046: Create Tribe Model and Repository
- **Description:** Implement Tribe model and repository.
- **Technical Context:**
  - Files: `internal/model/tribe.go`, `internal/repository/tribe_repo.go`
- **Acceptance Criteria:**
  - [ ] Tribe struct with i18n handling
  - [ ] List all tribes
  - [ ] FindByCode method
  - [ ] Parse JSONB for names

---

### [] T047: Implement Server Service
- **Description:** Create service for server operations.
- **Technical Context:**
  - Files: `internal/service/server_service.go`
- **Acceptance Criteria:**
  - [ ] List available servers (status = running)
  - [ ] Get server details with player count
  - [ ] Check if user already joined server

---

### [] T048: Implement Player Service
- **Description:** Create service for player operations.
- **Technical Context:**
  - Files: `internal/service/player_service.go`
- **Acceptance Criteria:**
  - [ ] JoinServer: create player, validate tribe, set protection
  - [ ] GetCurrentPlayer: return player with tribe info
  - [ ] UpdateSettings: name change with uniqueness check
  - [ ] Calculate beginner protection end date (7 days)

---

### [] T049: Implement Server Handlers
- **Description:** Create HTTP handlers for server endpoints.
- **Technical Context:**
  - Files: `internal/handler/server.go`
  - Endpoints: GET /servers, GET /servers/:id, POST /servers/:id/join
- **Acceptance Criteria:**
  - [ ] List servers with filtering
  - [ ] Get server details
  - [ ] Join server with tribe selection
  - [ ] Validation for tribe_id
  - [ ] Error for already joined

---

### [] T050: Implement Player Handlers
- **Description:** Create HTTP handlers for player endpoints.
- **Technical Context:**
  - Files: `internal/handler/player.go`
  - Endpoints: GET /players/me, PATCH /players/me, GET /tribes
- **Acceptance Criteria:**
  - [ ] Get current player info
  - [ ] Update player settings
  - [ ] List all tribes
  - [ ] Require active server selection

---

### [] T051: Create Server List Page (Frontend)
- **Description:** Create page to display available servers.
- **Technical Context:**
  - Files: `frontend/src/routes/servers/+page.svelte`
- **Acceptance Criteria:**
  - [ ] List all running servers
  - [ ] Show server name, region, status, player count
  - [ ] Indicate if user already joined
  - [ ] Click to join or enter server

---

### [] T052: Create Tribe Selection Page (Frontend)
- **Description:** Create page for selecting tribe when joining server.
- **Technical Context:**
  - Files: `frontend/src/routes/servers/[id]/join/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Show 3 tribes with descriptions
  - [ ] Display tribe bonuses
  - [ ] Tribe selection UI (cards)
  - [ ] Player name input
  - [ ] Submit to join server

---

### [] T053: Create Player Store (Frontend)
- **Description:** Create Svelte store for current player state.
- **Technical Context:**
  - Files: `frontend/src/lib/stores/player.ts`
- **Acceptance Criteria:**
  - [ ] Store player info, tribe, gold, VIP status
  - [ ] Fetch player on game entry
  - [ ] Update on changes
  - [ ] Selected server ID

---

# Phase 1: Village System

## 1.5 Village Management

### [] T054: Create Village Model and Repository
- **Description:** Implement Village model and repository.
- **Technical Context:**
  - Files: `internal/model/village.go`, `internal/repository/village_repo.go`
- **Acceptance Criteria:**
  - [ ] Village struct with resources as Decimal
  - [ ] Create, FindByID, FindByPlayer methods
  - [ ] Update resources with atomic operations
  - [ ] Find villages in area (x1,y1 to x2,y2)
  - [ ] Unit tests

---

### [] T055: Create Building Model and Repository
- **Description:** Implement Building model and repository.
- **Technical Context:**
  - Files: `internal/model/building.go`, `internal/repository/building_repo.go`
- **Acceptance Criteria:**
  - [ ] Building struct with upgrade status
  - [ ] FindByVillage, FindByID methods
  - [ ] StartUpgrade, CompleteUpgrade methods
  - [ ] Building queue operations for VIP
  - [ ] Unit tests

---

### [] T056: Create Building Definitions Data
- **Description:** Define building stats, costs, and requirements.
- **Technical Context:**
  - Files: `internal/game/building_data.go`
- **Acceptance Criteria:**
  - [ ] Struct for building definition
  - [ ] All 23 building types with base stats
  - [ ] Cost calculation per level (formula)
  - [ ] Build time calculation per level
  - [ ] Requirements (e.g., Barracks needs Main Building 3)

---

### [] T057: Implement Village Service
- **Description:** Create service for village operations.
- **Technical Context:**
  - Files: `internal/service/village_service.go`
- **Acceptance Criteria:**
  - [ ] CreateStartingVillage: random position, starter buildings
  - [ ] GetVillageDetails: with buildings and resources
  - [ ] UpdateResources: calculate based on production rate
  - [ ] RenameVillage with validation

---

### [] T058: Implement Building Service
- **Description:** Create service for building operations.
- **Technical Context:**
  - Files: `internal/service/building_service.go`
- **Acceptance Criteria:**
  - [ ] StartBuildingUpgrade: validate resources, requirements
  - [ ] CancelBuilding: refund resources (partial)
  - [ ] GetBuildingQueue: list pending upgrades
  - [ ] InstantComplete: spend gold to finish
  - [ ] Calculate upgrade effects on village

---

### [] T059: Implement Village Handlers
- **Description:** Create HTTP handlers for village endpoints.
- **Technical Context:**
  - Files: `internal/handler/village.go`
  - Endpoints: GET /villages, GET /villages/:id, PATCH /villages/:id
- **Acceptance Criteria:**
  - [ ] List player's villages
  - [ ] Get village details with updated resources
  - [ ] Update village (rename)
  - [ ] Validate village ownership

---

### [] T060: Implement Building Handlers
- **Description:** Create HTTP handlers for building endpoints.
- **Technical Context:**
  - Files: `internal/handler/building.go`
  - Endpoints: GET /villages/:id/buildings, POST /villages/:id/buildings
- **Acceptance Criteria:**
  - [ ] List buildings in village
  - [ ] Start building upgrade
  - [ ] Cancel building (DELETE)
  - [ ] Return updated queue and resources

---

### [] T061: Create Village Store (Frontend)
- **Description:** Create Svelte store for selected village.
- **Technical Context:**
  - Files: `frontend/src/lib/stores/village.ts`
- **Acceptance Criteria:**
  - [ ] Store current village with resources
  - [ ] Selected village ID
  - [ ] Buildings list
  - [ ] Fetch and refresh methods

---

### [] T062: Create Resource Store (Frontend)
- **Description:** Create Svelte store for live resource updates.
- **Technical Context:**
  - Files: `frontend/src/lib/stores/resources.ts`
- **Acceptance Criteria:**
  - [ ] Store wood, clay, iron, crop
  - [ ] Calculate client-side interpolation
  - [ ] Update from WebSocket events
  - [ ] Show overflow warning

---

### [] T063: Create ResourceBar Component
- **Description:** Create reusable resource bar UI component.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/ResourceBar.svelte`
- **Acceptance Criteria:**
  - [ ] Display 4 resource types with icons
  - [ ] Show current/max capacity
  - [ ] Color warning when near full
  - [ ] Production rate per hour
  - [ ] Animated counting effect

---

### [] T064: Create Village Overview Page
- **Description:** Create main village view page.
- **Technical Context:**
  - Files: `frontend/src/routes/game/village/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Display village name and coordinates
  - [ ] Resource bar at top
  - [ ] Building slots grid (village view)
  - [ ] Current construction queue
  - [ ] Quick actions (build, train)

---

### [] T065: Create BuildingSlot Component
- **Description:** Create component for building slot in village grid.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/BuildingSlot.svelte`
- **Acceptance Criteria:**
  - [ ] Display building icon and level
  - [ ] Show upgrade progress if building
  - [ ] Click to open building details
  - [ ] Empty slot shows "Build" button

---

### [] T066: Create Building Details Modal
- **Description:** Create modal for building information and actions.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/BuildingModal.svelte`
- **Acceptance Criteria:**
  - [ ] Show building name, level, description
  - [ ] Upgrade button with cost
  - [ ] Requirements check
  - [ ] Queue position for VIP
  - [ ] Production stats (for resource buildings)

---

### [] T067: Create Buildings List Page
- **Description:** Create page listing all buildings in village.
- **Technical Context:**
  - Files: `frontend/src/routes/game/village/buildings/+page.svelte`
- **Acceptance Criteria:**
  - [ ] List all buildings with levels
  - [ ] Filter by category (resource, military, infrastructure)
  - [ ] Quick upgrade button
  - [ ] Construction queue display

---

# Phase 1: Troop System

## 1.6 Troop Training

### [] T068: Create Troop Model and Repository
- **Description:** Implement Troop and TroopQueue models.
- **Technical Context:**
  - Files: `internal/model/troop.go`, `internal/repository/troop_repo.go`
- **Acceptance Criteria:**
  - [ ] Troop struct with count per type
  - [ ] TroopQueue struct for training
  - [ ] CRUD operations
  - [ ] Get troops by village
  - [ ] Unit tests

---

### [] T069: Create Troop Definitions Data
- **Description:** Define troop stats and costs.
- **Technical Context:**
  - Files: `internal/game/troop_data.go`
- **Acceptance Criteria:**
  - [ ] Struct for troop definition
  - [ ] All 16 troop types with stats
  - [ ] Training time and costs
  - [ ] Crop consumption per troop
  - [ ] Required building per type

---

### [] T070: Implement Troop Service
- **Description:** Create service for troop operations.
- **Technical Context:**
  - Files: `internal/service/troop_service.go`
- **Acceptance Criteria:**
  - [ ] GetVillageTroops: list all troops
  - [ ] StartTraining: validate resources, add to queue
  - [ ] CancelTraining: refund resources
  - [ ] GetTrainingQueue: with remaining time
  - [ ] Check crop consumption capacity

---

### [] T071: Implement Troop Handlers
- **Description:** Create HTTP handlers for troop endpoints.
- **Technical Context:**
  - Files: `internal/handler/troop.go`
  - Endpoints: GET /villages/:id/troops, POST /villages/:id/troops
- **Acceptance Criteria:**
  - [ ] List troops in village
  - [ ] Start troop training
  - [ ] Cancel training (DELETE)
  - [ ] Return queue and resources

---

### [] T072: Create TroopCard Component
- **Description:** Create component for displaying troop info.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/TroopCard.svelte`
- **Acceptance Criteria:**
  - [ ] Display troop icon and name
  - [ ] Show stats (attack, defense, speed, carry)
  - [ ] Training cost
  - [ ] Train button with quantity input

---

### [] T073: Create Troops Page
- **Description:** Create page for viewing and training troops.
- **Technical Context:**
  - Files: `frontend/src/routes/game/village/troops/+page.svelte`
- **Acceptance Criteria:**
  - [ ] List available troops for tribe
  - [ ] Current garrison count
  - [ ] Training queue with timer
  - [ ] Train form with quantity
  - [ ] Crop consumption warning

---

### [] T074: Create Timer Component
- **Description:** Create reusable countdown timer component.
- **Technical Context:**
  - Files: `frontend/src/lib/components/ui/Timer.svelte`
- **Acceptance Criteria:**
  - [ ] Countdown from target timestamp
  - [ ] Format HH:MM:SS or D H M S
  - [ ] Callback on complete
  - [ ] Pause/resume support
  - [ ] Instant complete button (gold)

---

# Phase 1: Map System

## 1.7 World Map

### [] T075: Create MapTile Model and Repository
- **Description:** Implement MapTile model for terrain data.
- **Technical Context:**
  - Files: `internal/model/map_tile.go`, `internal/repository/map_repo.go`
- **Acceptance Criteria:**
  - [ ] MapTile struct with terrain, oasis
  - [ ] GetTilesInArea: fetch tiles in viewport
  - [ ] GetTileAt: get single tile
  - [ ] Bulk insert for map generation

---

### [] T076: Create Map Generation Script
- **Description:** Create script to generate map tiles for a server.
- **Technical Context:**
  - Files: `scripts/generate_map.go`
- **Acceptance Criteria:**
  - [ ] Generate 200x200 grid
  - [ ] Random terrain distribution
  - [ ] SEA-inspired shape (more water in archipelago)
  - [ ] Oases at random locations
  - [ ] Seed-based for reproducibility

---

### [] T077: Implement Map Service
- **Description:** Create service for map operations.
- **Technical Context:**
  - Files: `internal/service/map_service.go`
- **Acceptance Criteria:**
  - [ ] GetMapData: tiles + villages in viewport
  - [ ] GetTileDetails: tile info + village if present
  - [ ] SearchMap: find by player/village name
  - [ ] Calculate distance between coordinates

---

### [] T078: Implement Map Handlers
- **Description:** Create HTTP handlers for map endpoints.
- **Technical Context:**
  - Files: `internal/handler/map.go`
  - Endpoints: GET /map, GET /map/:x/:y, GET /map/search
- **Acceptance Criteria:**
  - [ ] Get map tiles with viewport params (x, y, width, height)
  - [ ] Get single tile details
  - [ ] Search by name
  - [ ] Return terrain, village info, owner

---

### [] T079: Create MapTile Component
- **Description:** Create component for single map tile.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/MapTile.svelte`
- **Acceptance Criteria:**
  - [ ] Display terrain type (color/icon)
  - [ ] Show village if present
  - [ ] Owner indicator (self, ally, enemy)
  - [ ] Hover tooltip with details
  - [ ] Click to open tile actions

---

### [] T080: Create Map Page
- **Description:** Create world map page with viewport.
- **Technical Context:**
  - Files: `frontend/src/routes/game/map/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Grid display of tiles (e.g., 15x15 viewport)
  - [ ] Pan with arrow keys or drag
  - [ ] Zoom levels
  - [ ] Centered on selected village
  - [ ] Coordinate display

---

### [] T081: Create Map Search Component
- **Description:** Create search box for finding locations.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/MapSearch.svelte`
- **Acceptance Criteria:**
  - [ ] Search input with autocomplete
  - [ ] Search by coordinates (x|y)
  - [ ] Search by player/village name
  - [ ] Jump to location on select

---

### [] T082: Create Tile Details Modal
- **Description:** Create modal for tile information and actions.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/TileModal.svelte`
- **Acceptance Criteria:**
  - [ ] Show terrain type and bonuses
  - [ ] Village info if present
  - [ ] Send army button
  - [ ] Distance from current village
  - [ ] Scout button

---

# Phase 1: Combat System

## 1.8 Army Movement & Combat

### [] T083: Create Army Model and Repository
- **Description:** Implement Army model for moving troops.
- **Technical Context:**
  - Files: `internal/model/army.go`, `internal/repository/army_repo.go`
- **Acceptance Criteria:**
  - [ ] Army struct with JSONB troops/resources
  - [ ] Create, FindByID, FindByPlayer methods
  - [ ] Get arriving armies in time window
  - [ ] Get incoming attacks to village
  - [ ] Unit tests

---

### [] T084: Implement Combat Formulas
- **Description:** Create combat calculation formulas.
- **Technical Context:**
  - Files: `internal/game/combat.go`, `internal/game/formula.go`
- **Acceptance Criteria:**
  - [ ] Attack power calculation
  - [ ] Defense power (infantry vs cavalry)
  - [ ] Wall defense bonus
  - [ ] Troop loss calculation
  - [ ] Resource capture calculation
  - [ ] Unit tests with edge cases

---

### [] T085: Implement Army Service
- **Description:** Create service for army operations.
- **Technical Context:**
  - Files: `internal/service/army_service.go`
- **Acceptance Criteria:**
  - [ ] SendArmy: validate troops, calculate travel time
  - [ ] RecallArmy: if not arrived yet
  - [ ] GetPlayerArmies: outgoing and returning
  - [ ] SimulateBattle: preview combat result
  - [ ] Travel time based on slowest troop

---

### [] T086: Implement Combat Service
- **Description:** Create service for battle execution.
- **Technical Context:**
  - Files: `internal/service/combat_service.go`
- **Acceptance Criteria:**
  - [ ] ExecuteBattle: calculate result, update troops
  - [ ] Raid: steal resources based on carry capacity
  - [ ] Attack: destroy buildings, lower loyalty
  - [ ] Support: reinforce village
  - [ ] Create battle report

---

### [] T087: Implement Army Handlers
- **Description:** Create HTTP handlers for army endpoints.
- **Technical Context:**
  - Files: `internal/handler/army.go`
  - Endpoints: GET /armies, POST /armies, DELETE /armies/:id
- **Acceptance Criteria:**
  - [ ] List player's armies
  - [ ] Send army (mission type, troops, target)
  - [ ] Recall army
  - [ ] Simulate battle endpoint

---

### [] T088: Create Army Overview Page
- **Description:** Create page showing all player armies.
- **Technical Context:**
  - Files: `frontend/src/routes/game/army/+page.svelte`
- **Acceptance Criteria:**
  - [ ] List outgoing armies with destination
  - [ ] List returning armies
  - [ ] Show arrival countdown
  - [ ] Recall button for outgoing

---

### [] T089: Create Send Army Page
- **Description:** Create page for sending troops.
- **Technical Context:**
  - Files: `frontend/src/routes/game/army/send/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Target selection (coordinates or from map)
  - [ ] Mission type selection (raid, attack, support)
  - [ ] Troop selection with available counts
  - [ ] Travel time preview
  - [ ] Resource carry capacity display

---

### [] T090: Create ArmyRow Component
- **Description:** Create component for army list item.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/ArmyRow.svelte`
- **Acceptance Criteria:**
  - [ ] Show source and destination
  - [ ] Mission type icon
  - [ ] Troop summary
  - [ ] Arrival countdown
  - [ ] Recall button

---

# Phase 2: Game Engine

## 2.1 Tick Engine

### [] T091: Create Game Tick Engine
- **Description:** Implement main game tick engine that runs periodic updates.
- **Technical Context:**
  - Files: `internal/game/engine.go`
- **Acceptance Criteria:**
  - [ ] Start/Stop methods
  - [ ] Configurable tick interval
  - [ ] Separate goroutines for tick types
  - [ ] Error recovery and logging
  - [ ] Graceful shutdown

---

### [] T092: Implement Resource Tick
- **Description:** Create resource production tick (every 1 minute).
- **Technical Context:**
  - Files: `internal/game/resource.go`
- **Acceptance Criteria:**
  - [ ] Batch update all villages
  - [ ] Calculate production based on rates
  - [ ] Cap at warehouse/granary capacity
  - [ ] Handle negative crop production
  - [ ] Trigger WebSocket updates

---

### [] T093: Implement Building Tick
- **Description:** Create building completion tick (every 1 second).
- **Technical Context:**
  - Files: `internal/game/building_tick.go`
- **Acceptance Criteria:**
  - [ ] Check buildings with ends_at <= now
  - [ ] Complete upgrade, update level
  - [ ] Update village stats (production, capacity)
  - [ ] Start next item in queue (VIP)
  - [ ] Send completion notification

---

### [] T094: Implement Troop Training Tick
- **Description:** Create troop training completion tick (every 1 second).
- **Technical Context:**
  - Files: `internal/game/troop_tick.go`
- **Acceptance Criteria:**
  - [ ] Check training queues with ends_at <= now
  - [ ] Add troops to garrison
  - [ ] Update crop consumption
  - [ ] Start next training batch
  - [ ] Send completion notification

---

### [] T095: Implement Army Arrival Tick
- **Description:** Create army arrival processing tick (every 1 second).
- **Technical Context:**
  - Files: `internal/game/army_tick.go`
- **Acceptance Criteria:**
  - [ ] Check armies with arrives_at <= now
  - [ ] Execute combat or support
  - [ ] Update returning army
  - [ ] Create reports for both sides
  - [ ] Send attack notification

---

### [] T096: Implement Starvation Tick
- **Description:** Create starvation processing tick (every 5 minutes).
- **Technical Context:**
  - Files: `internal/game/starvation.go`
- **Acceptance Criteria:**
  - [ ] Find villages with negative crop
  - [ ] Calculate troops to kill
  - [ ] Kill cheapest troops first
  - [ ] Create starvation report
  - [ ] Send warning notification before

---

# Phase 2: Real-time System

## 2.2 WebSocket & Notifications

### [] T097: Create WebSocket Hub
- **Description:** Implement WebSocket connection hub.
- **Technical Context:**
  - Files: `internal/realtime/hub.go`
- **Acceptance Criteria:**
  - [ ] Client registration/unregistration
  - [ ] Channel subscription system
  - [ ] Broadcast to channel
  - [ ] Send to specific client
  - [ ] Ping/pong heartbeat

---

### [] T098: Create WebSocket Client Handler
- **Description:** Implement WebSocket client connection handler.
- **Technical Context:**
  - Files: `internal/realtime/client.go`
- **Acceptance Criteria:**
  - [ ] Authenticate on connect (JWT)
  - [ ] Message read/write goroutines
  - [ ] Handle subscribe/unsubscribe
  - [ ] Auto-disconnect on idle
  - [ ] Reconnection token support

---

### [] T099: Define WebSocket Events
- **Description:** Define all WebSocket event types and payloads.
- **Technical Context:**
  - Files: `internal/realtime/events.go`
- **Acceptance Criteria:**
  - [ ] Event struct definitions
  - [ ] resource_update event
  - [ ] building_complete, troop_complete events
  - [ ] under_attack, army_arrived events
  - [ ] message_received event

---

### [] T100: Integrate WebSocket with Handlers
- **Description:** Add WebSocket endpoint to HTTP server.
- **Technical Context:**
  - Files: `internal/handler/websocket.go`, `internal/server/routes.go`
- **Acceptance Criteria:**
  - [ ] Upgrade HTTP to WebSocket
  - [ ] Pass auth token in query param
  - [ ] Register with hub
  - [ ] Handle disconnection cleanup

---

### [] T101: Integrate WebSocket with Game Engine
- **Description:** Trigger WebSocket events from game ticks.
- **Technical Context:**
  - Files: Update `internal/game/*.go` files
- **Acceptance Criteria:**
  - [ ] Resource updates trigger events
  - [ ] Building completion triggers event
  - [ ] Incoming attack triggers event
  - [ ] Batch events for efficiency

---

### [] T102: Create Notification Service
- **Description:** Create service for managing notifications.
- **Technical Context:**
  - Files: `internal/service/notification_service.go`
- **Acceptance Criteria:**
  - [ ] Queue notification
  - [ ] Send via WebSocket
  - [ ] Fallback to database for offline
  - [ ] Mark as read
  - [ ] Get unread count

---

### [] T103: Create Notification Store (Frontend)
- **Description:** Create Svelte store for notifications.
- **Technical Context:**
  - Files: `frontend/src/lib/stores/notifications.ts`
- **Acceptance Criteria:**
  - [ ] Store notification list
  - [ ] Add from WebSocket events
  - [ ] Toast display queue
  - [ ] Unread count
  - [ ] Mark as read action

---

### [] T104: Create Toast Component
- **Description:** Create toast notification component.
- **Technical Context:**
  - Files: `frontend/src/lib/components/ui/Toast.svelte`
- **Acceptance Criteria:**
  - [ ] Multiple toast types (success, error, warning, info)
  - [ ] Auto-dismiss with timer
  - [ ] Click to dismiss
  - [ ] Stack multiple toasts
  - [ ] Animation enter/exit

---

# Phase 2: Alliance System

## 2.3 Alliance Features

### [] T105: Create Alliance Model and Repository
- **Description:** Implement Alliance and AllianceMember models.
- **Technical Context:**
  - Files: `internal/model/alliance.go`, `internal/repository/alliance_repo.go`
- **Acceptance Criteria:**
  - [ ] Alliance struct with all fields
  - [ ] AllianceMember struct with role
  - [ ] CRUD operations
  - [ ] Get members by alliance
  - [ ] Check player membership

---

### [] T106: Implement Alliance Service
- **Description:** Create service for alliance operations.
- **Technical Context:**
  - Files: `internal/service/alliance_service.go`
- **Acceptance Criteria:**
  - [ ] CreateAlliance: validate name, set leader
  - [ ] InvitePlayer: create invitation
  - [ ] JoinAlliance: accept invitation
  - [ ] LeaveAlliance: handle leader transfer
  - [ ] KickMember: with role check
  - [ ] ChangeRole: promote/demote

---

### [] T107: Implement Alliance Handlers
- **Description:** Create HTTP handlers for alliance endpoints.
- **Technical Context:**
  - Files: `internal/handler/alliance.go`
- **Acceptance Criteria:**
  - [ ] All endpoints from API spec
  - [ ] Role-based authorization
  - [ ] Member limit enforcement
  - [ ] Unique name/tag per server

---

### [] T108: Create Alliance Home Page
- **Description:** Create alliance overview page.
- **Technical Context:**
  - Files: `frontend/src/routes/game/alliance/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Alliance name, tag, description
  - [ ] Member count / max
  - [ ] Bank balance
  - [ ] Recent activity
  - [ ] Create alliance if not member

---

### [] T109: Create Alliance Members Page
- **Description:** Create page for managing alliance members.
- **Technical Context:**
  - Files: `frontend/src/routes/game/alliance/members/+page.svelte`
- **Acceptance Criteria:**
  - [ ] List members with role
  - [ ] Sort by population
  - [ ] Invite button (officers+)
  - [ ] Kick button (officers+)
  - [ ] Promote/demote (leader only)

---

### [] T110: Create Alliance Diplomacy Page
- **Description:** Create page for NAP and war declarations.
- **Technical Context:**
  - Files: `frontend/src/routes/game/alliance/diplomacy/+page.svelte`
- **Acceptance Criteria:**
  - [ ] List current relations (NAP, Ally, War)
  - [ ] Add new relation (diplomats+)
  - [ ] Cancel relation
  - [ ] Pending requests

---

### [] T111: Create Alliance Chat Page
- **Description:** Create alliance chat page.
- **Technical Context:**
  - Files: `frontend/src/routes/game/alliance/chat/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Real-time message display
  - [ ] Send message input
  - [ ] Load older messages (pagination)
  - [ ] Member mentions
  - [ ] WebSocket integration

---

# Phase 2: Reports & Messages

## 2.4 Communication

### [] T112: Create Report Model and Repository
- **Description:** Implement Report model for battle/trade/scout reports.
- **Technical Context:**
  - Files: `internal/model/report.go`, `internal/repository/report_repo.go`
- **Acceptance Criteria:**
  - [ ] Report struct with JSONB data
  - [ ] Create, FindByID, FindByPlayer methods
  - [ ] Mark as read
  - [ ] Delete report
  - [ ] Get unread count

---

### [] T113: Implement Report Service
- **Description:** Create service for report operations.
- **Technical Context:**
  - Files: `internal/service/report_service.go`
- **Acceptance Criteria:**
  - [ ] CreateBattleReport: from combat result
  - [ ] CreateScoutReport: from scout mission
  - [ ] GetReports: with filtering
  - [ ] MarkAsRead, DeleteReport

---

### [] T114: Implement Report Handlers
- **Description:** Create HTTP handlers for report endpoints.
- **Technical Context:**
  - Files: `internal/handler/report.go`
- **Acceptance Criteria:**
  - [ ] List reports with pagination
  - [ ] Get report details
  - [ ] Mark as read
  - [ ] Delete report
  - [ ] Filter by type

---

### [] T115: Create Reports List Page
- **Description:** Create page listing all reports.
- **Technical Context:**
  - Files: `frontend/src/routes/game/reports/+page.svelte`
- **Acceptance Criteria:**
  - [ ] List reports with type icon
  - [ ] Unread indicator
  - [ ] Filter by type
  - [ ] Date formatting
  - [ ] Click to view details

---

### [] T116: Create Report Details Page
- **Description:** Create page for viewing report details.
- **Technical Context:**
  - Files: `frontend/src/routes/game/reports/[id]/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Battle report: troops, losses, loot
  - [ ] Scout report: enemy troops, buildings
  - [ ] Trade report: resources exchanged
  - [ ] Reply/attack buttons

---

### [] T117: Create Message Model and Repository
- **Description:** Implement Message model for private/alliance messages.
- **Technical Context:**
  - Files: `internal/model/message.go`, `internal/repository/message_repo.go`
- **Acceptance Criteria:**
  - [ ] Message struct
  - [ ] Create, FindByRecipient, FindByAlliance methods
  - [ ] Mark as read
  - [ ] Pagination support

---

### [] T118: Implement Message Handlers
- **Description:** Create HTTP handlers for message endpoints.
- **Technical Context:**
  - Files: `internal/handler/message.go`
- **Acceptance Criteria:**
  - [ ] List messages
  - [ ] Send private message
  - [ ] Mark as read
  - [ ] Get conversation thread

---

### [] T119: Create Messages Page
- **Description:** Create private messages page.
- **Technical Context:**
  - Files: `frontend/src/routes/game/messages/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Inbox list
  - [ ] Compose new message
  - [ ] Reply to message
  - [ ] Unread count
  - [ ] Player search for recipient

---

# Phase 2: Shop & Payment

## 2.5 Monetization

### [] T120: Create Transaction Model and Repository
- **Description:** Implement Transaction model for payment history.
- **Technical Context:**
  - Files: `internal/model/transaction.go`, `internal/repository/transaction_repo.go`
- **Acceptance Criteria:**
  - [ ] Transaction struct with all fields
  - [ ] Create, UpdateStatus methods
  - [ ] Find pending transactions
  - [ ] User transaction history

---

### [] T121: Implement Shop Service
- **Description:** Create service for shop operations.
- **Technical Context:**
  - Files: `internal/service/shop_service.go`
- **Acceptance Criteria:**
  - [ ] ListShopItems: gold packages, VIP, skins
  - [ ] InitiatePurchase: create pending transaction
  - [ ] CompletePurchase: add gold/VIP to account
  - [ ] UseGold: instant complete building/training
  - [ ] RefundPurchase: handle chargebacks

---

### [] T122: Implement Payment Webhook Handler
- **Description:** Create webhook handler for payment provider callbacks.
- **Technical Context:**
  - Files: `internal/handler/shop.go`
  - Providers: Omise (Thai), Stripe (international)
- **Acceptance Criteria:**
  - [ ] Verify webhook signature
  - [ ] Handle success callback
  - [ ] Handle failure callback
  - [ ] Idempotent processing
  - [ ] Log all events

---

### [] T123: Implement Shop Handlers
- **Description:** Create HTTP handlers for shop endpoints.
- **Technical Context:**
  - Files: `internal/handler/shop.go`
- **Acceptance Criteria:**
  - [ ] List shop items
  - [ ] Initiate purchase
  - [ ] Get transaction history
  - [ ] Use gold for instant complete
  - [ ] Subscribe to VIP

---

### [] T124: Create Shop Page
- **Description:** Create in-game shop page.
- **Technical Context:**
  - Files: `frontend/src/routes/game/shop/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Gold packages with prices
  - [ ] VIP subscription option
  - [ ] Current gold balance
  - [ ] Purchase history
  - [ ] Payment method selection

---

### [] T125: Implement VIP Benefits
- **Description:** Implement VIP-specific features throughout the app.
- **Technical Context:**
  - Files: Update various service files
- **Acceptance Criteria:**
  - [ ] Multi-queue building (2-3 slots)
  - [ ] Larger map view
  - [ ] Auto-evade toggle
  - [ ] Advanced statistics
  - [ ] VIP indicator in UI

---

# Phase 3: Polish & Optimization

## 3.1 Final Features

### [] T126: Implement Full i18n Support
- **Description:** Complete internationalization for all text.
- **Technical Context:**
  - Files: `frontend/src/lib/i18n/locales/*.json`
- **Acceptance Criteria:**
  - [ ] All UI text translated (TH, EN)
  - [ ] Date/number formatting localized
  - [ ] Language switcher in settings
  - [ ] Language preference persisted
  - [ ] RTL support prepared (future)

---

### [] T127: Create Game Dashboard
- **Description:** Create main dashboard page after login.
- **Technical Context:**
  - Files: `frontend/src/routes/game/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Overview of all villages
  - [ ] Incoming attacks warning
  - [ ] Active construction/training
  - [ ] Recent reports summary
  - [ ] Alliance news

---

### [] T128: Create Settings Page
- **Description:** Create user settings page.
- **Technical Context:**
  - Files: `frontend/src/routes/game/settings/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Language selection
  - [ ] Notification preferences
  - [ ] Change password
  - [ ] Link/unlink OAuth
  - [ ] Delete account

---

### [] T129: Create Game Layout with Sidebar
- **Description:** Create main game layout with navigation.
- **Technical Context:**
  - Files: `frontend/src/routes/game/+layout.svelte`
- **Acceptance Criteria:**
  - [ ] Top navbar with resources
  - [ ] Left sidebar with navigation
  - [ ] Village selector dropdown
  - [ ] Notification bell
  - [ ] Mobile responsive drawer

---

### [] T130: Create Landing Page
- **Description:** Create public landing page for the game.
- **Technical Context:**
  - Files: `frontend/src/routes/+page.svelte`
- **Acceptance Criteria:**
  - [ ] Game introduction
  - [ ] Tribe showcase
  - [ ] Screenshots/videos
  - [ ] Register/Login buttons
  - [ ] SEO meta tags

---

### [] T131: Implement Error Handling & Loading States
- **Description:** Add consistent error and loading UI throughout app.
- **Technical Context:**
  - Files: Various frontend components
- **Acceptance Criteria:**
  - [ ] Loading spinners for async operations
  - [ ] Error boundaries
  - [ ] Retry buttons for failed requests
  - [ ] Form validation feedback
  - [ ] Toast for operation results

---

### [] T132: Performance Optimization - Backend
- **Description:** Optimize backend performance.
- **Technical Context:**
  - Files: Various backend files
- **Acceptance Criteria:**
  - [ ] Add database indexes (based on query analysis)
  - [ ] Implement response caching
  - [ ] Batch database operations in ticks
  - [ ] Connection pool tuning
  - [ ] Profile and fix slow queries

---

### [] T133: Performance Optimization - Frontend
- **Description:** Optimize frontend performance.
- **Technical Context:**
  - Files: Various frontend files
- **Acceptance Criteria:**
  - [ ] Code splitting per route
  - [ ] Image optimization
  - [ ] Cache static assets
  - [ ] Reduce bundle size
  - [ ] Lighthouse score > 90

---

### [] T134: Security Audit & Hardening
- **Description:** Conduct security review and implement fixes.
- **Technical Context:**
  - Files: Various files
- **Acceptance Criteria:**
  - [ ] Input sanitization review
  - [ ] SQL injection prevention verified
  - [ ] XSS prevention verified
  - [ ] CSRF tokens for mutations
  - [ ] Rate limiting tested

---

### [] T135: Mobile Responsiveness
- **Description:** Ensure all pages work on mobile devices.
- **Technical Context:**
  - Files: Various frontend files
- **Acceptance Criteria:**
  - [ ] All pages render correctly on mobile
  - [ ] Touch-friendly UI elements
  - [ ] Responsive navigation
  - [ ] Map works with touch gestures
  - [ ] Tested on iOS and Android browsers

---

### [] T136: Create E2E Test Suite
- **Description:** Create end-to-end tests for critical flows.
- **Technical Context:**
  - Files: `frontend/tests/e2e/*.spec.ts`
  - Tool: Playwright
- **Acceptance Criteria:**
  - [ ] Registration flow test
  - [ ] Login flow test
  - [ ] Join server flow test
  - [ ] Build building flow test
  - [ ] Send army flow test

---

### [] T137: Setup CI/CD Pipeline
- **Description:** Create GitHub Actions workflow for CI/CD.
- **Technical Context:**
  - Files: `.github/workflows/*.yml`
- **Acceptance Criteria:**
  - [ ] Run tests on PR
  - [ ] Run linters on PR
  - [ ] Build Docker images
  - [ ] Deploy to staging on develop
  - [ ] Deploy to production on main

---

### [] T138: Create Production Dockerfile
- **Description:** Create optimized Dockerfiles for production.
- **Technical Context:**
  - Files: `backend/Dockerfile`, `frontend/Dockerfile`
- **Acceptance Criteria:**
  - [ ] Multi-stage build
  - [ ] Minimal final image size
  - [ ] Non-root user
  - [ ] Health check endpoint
  - [ ] Environment variable support

---

### [] T139: Setup Monitoring & Alerting
- **Description:** Configure monitoring with Prometheus and Grafana.
- **Technical Context:**
  - Files: `infra/prometheus.yml`, `infra/grafana/`
- **Acceptance Criteria:**
  - [ ] Metrics endpoint in Go server
  - [ ] Prometheus scrape config
  - [ ] Grafana dashboards
  - [ ] Alert rules for critical metrics
  - [ ] Sentry error tracking

---

### [] T140: Documentation & README
- **Description:** Create comprehensive documentation.
- **Technical Context:**
  - Files: `README.md`, `docs/*.md`
- **Acceptance Criteria:**
  - [ ] Setup instructions
  - [ ] API documentation
  - [ ] Environment variables list
  - [ ] Deployment guide
  - [ ] Contributing guidelines

---

# Summary

| Phase | Module | Tasks | Priority |
|-------|--------|-------|----------|
| 1 | Project Setup | T001-T014 | Critical |
| 1 | Database Schema | T015-T030 | Critical |
| 1 | Authentication | T031-T043 | Critical |
| 1 | Server & Player | T044-T053 | Critical |
| 1 | Village System | T054-T067 | Critical |
| 1 | Troop System | T068-T074 | Critical |
| 1 | Map System | T075-T082 | Critical |
| 1 | Combat System | T083-T090 | Critical |
| 2 | Game Engine | T091-T096 | High |
| 2 | Real-time System | T097-T104 | High |
| 2 | Alliance System | T105-T111 | High |
| 2 | Reports & Messages | T112-T119 | High |
| 2 | Shop & Payment | T120-T125 | High |
| 3 | Polish & Optimization | T126-T140 | Medium |

**Total Tasks: 140**

---

*Document Version: 1.0*
*Last Updated: December 2025*
*Author: Engineering Team*
