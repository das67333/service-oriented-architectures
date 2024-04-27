package main

import (
	"fmt"
	"log"
	"net"
	"os"
	"time"

	pb "service-posts/protos"

	grpc "google.golang.org/grpc"

	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
)

type Server struct {
	pb.UnimplementedServicePostsServer
	db sqlx.DB
}

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

func runGrpcServer(db *sqlx.DB) {
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	pb.RegisterServicePostsServer(s, &Server{db: *db})
	log.Printf("gRPC server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}

func main() {
	db := connectDb()
	TryCreateTable(&db)
	defer db.Close()
	runGrpcServer(&db)
}
