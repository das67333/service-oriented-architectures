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

// Server is used to implement helloworld.GreeterServer.
type Server struct {
	pb.UnimplementedServicePostsServer
	db sqlx.DB
}

func connectDb() sqlx.DB {
	const TIMEOUT = 100 * time.Millisecond

	dsn := fmt.Sprintf(
		"user=%s password=%s host=%s sslmode=disable",
		os.Getenv("POSTS_DB_USER"),
		os.Getenv("POSTS_DB_PASSWORD"),
		os.Getenv("POSTS_DB_HOST"),
	)
	var db *sqlx.DB
	for {
		try_db, err := sqlx.Connect("postgres", dsn)
		if err == nil {
			db = try_db
			break
		}
		log.Printf(
			"Cannot connect to database: %v. Attempting to reconnect...",
			err,
		)
		time.Sleep(TIMEOUT)
	}
	fmt.Println("Connected to the database")
	return *db
}

func runGrpcServer(db *sqlx.DB) {
	port := os.Getenv("POSTS_GRPC_PORT")
	lis, err := net.Listen("tcp", ":"+port)
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
