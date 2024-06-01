package main

import (
	"fmt"
	"log"
	"os"
	"service-posts/server"
	"time"

	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
)

func connectDb() sqlx.DB {
	const INITIAL_TIMEOUT = 1_000 * time.Millisecond
	const TIMEOUT_MULTIPLIER = 1.2

	dsn := fmt.Sprintf(
		"user=postgres password=%s host=posts_db sslmode=disable",
		os.Getenv("POSTS_DB_PASSWORD"),
	)
	var db *sqlx.DB
	timeout := INITIAL_TIMEOUT
	for {
		try_db, err := sqlx.Connect("postgres", dsn)
		if err == nil {
			db = try_db
			break
		}
		log.Printf(
			"Cannot connect to database: \"%v\". Reconnecting in %.1f seconds...",
			err,
			timeout.Seconds(),
		)
		time.Sleep(timeout)
		timeout = time.Duration(float64(timeout) * TIMEOUT_MULTIPLIER)
	}
	fmt.Println("Connected to the database")
	return *db
}

func main() {
	db := connectDb()
	defer db.Close()
	server.InitDb(&db)
	if err := server.RunGrpcServer(&db); err != nil {
		log.Fatalf("Failed to run gRPC server: %v", err)
	}
}
