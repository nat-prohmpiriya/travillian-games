package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/travillian/tusk-horn/internal/config"
)

func main() {
	cfg, err := config.Load()
	if err != nil {
		log.Fatalf("Failed to load config: %v", err)
	}

	// Connect to default 'postgres' database to create new db
	dsn := fmt.Sprintf("postgres://%s:%s@%s:%s/postgres?sslmode=%s",
		cfg.Postgres.User,
		cfg.Postgres.Password,
		cfg.Postgres.Host,
		cfg.Postgres.Port,
		cfg.Postgres.SSLMode,
	)

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	conn, err := pgx.Connect(ctx, dsn)
	if err != nil {
		log.Fatalf("Failed to connect to postgres maintenance db: %v", err)
	}
	defer conn.Close(ctx)

	targetDB := cfg.Postgres.DBName
	if targetDB == "" {
		targetDB = "tusk_horn"
	}

	// Check if db exists
	var exists bool
	err = conn.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)", targetDB).Scan(&exists)
	if err != nil {
		log.Fatalf("Failed to check if db exists: %v", err)
	}

	if !exists {
		log.Printf("Database %s does not exist. Creating...", targetDB)
		_, err = conn.Exec(ctx, fmt.Sprintf("CREATE DATABASE %s", targetDB))
		if err != nil {
			log.Fatalf("Failed to create database: %v", err)
		}
		log.Printf("Database %s created successfully.", targetDB)
	} else {
		log.Printf("Database %s already exists.", targetDB)
	}
}
