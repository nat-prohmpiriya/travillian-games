package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/travillian/tusk-horn/internal/config"
	"github.com/travillian/tusk-horn/internal/pkg/firebase"
)

func main() {
	log.Println("Tusk & Horn Server Starting...")

	// 1. Load Config
	cfg, err := config.Load()
	if err != nil {
		log.Fatalf("Failed to load config: %v", err)
	}

	// 2. Initialize Firebase
	// Allow empty path to rely on GOOGLE_APPLICATION_CREDENTIALS env in local
	fbClient, err := firebase.NewClient(cfg.Firebase.CredentialsPath)
	if err != nil {
		log.Printf("Warning: Failed to init Firebase: %v. Auth will fail.", err)
	} else {
		_ = fbClient // Use in handlers later
	}

	// 3. Setup Router
	r := chi.NewRouter()
	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)

	r.Get("/health", func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("OK"))
	})

	// 4. Start Server
	port := cfg.Server.Port
	if port == "" {
		port = "8080"
	}

	addr := fmt.Sprintf(":%s", port)
	log.Printf("Server listening on %s", addr)
	if err := http.ListenAndServe(addr, r); err != nil {
		log.Fatalf("Server failed: %v", err)
	}
}
