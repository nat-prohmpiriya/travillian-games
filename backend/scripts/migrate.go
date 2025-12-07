package main

import (
	"flag"
	"fmt"
	"log"

	"github.com/golang-migrate/migrate/v4"
	_ "github.com/golang-migrate/migrate/v4/database/postgres"
	_ "github.com/golang-migrate/migrate/v4/source/file"
	"github.com/travillian/tusk-horn/internal/config"
)

func main() {
	direction := flag.String("direction", "up", "migration direction (up/down)")
	flag.Parse()

	cfg, err := config.Load()
	if err != nil {
		log.Fatalf("Failed to load config: %v", err)
	}

	dsn := fmt.Sprintf("postgres://%s:%s@%s:%s/%s?sslmode=%s",
		cfg.Postgres.User,
		cfg.Postgres.Password,
		cfg.Postgres.Host,
		cfg.Postgres.Port,
		cfg.Postgres.DBName,
		cfg.Postgres.SSLMode,
	)

	m, err := migrate.New(
		"file://migrations",
		dsn,
	)
	if err != nil {
		log.Fatalf("Failed to create migrate instance: %v", err)
	}

	if *direction == "down" {
		if err := m.Down(); err != nil && err != migrate.ErrNoChange {
			log.Fatalf("Migration down failed: %v", err)
		}
		log.Println("Migration down successful")
	} else {
		if err := m.Up(); err != nil && err != migrate.ErrNoChange {
			log.Fatalf("Migration up failed: %v", err)
		}
		log.Println("Migration up successful")
	}
}
