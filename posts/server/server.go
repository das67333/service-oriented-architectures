package server

import (
	"fmt"
	"log"
	"net"
	"os"
	pb "service-posts/protos"

	"github.com/jmoiron/sqlx"
	"google.golang.org/grpc"
)

func InitDb(db *sqlx.DB) {
	s, err := os.ReadFile("init.sql")
	if err != nil {
		log.Fatalf("Failed to read init.sql: %v", err)
	}
	db.MustExec(string(s))
}

type Server struct {
	pb.UnimplementedServicePostsServer
	db sqlx.DB
}

func RunGrpcServer(db *sqlx.DB) error {
	if db == nil {
		return fmt.Errorf("db is nil")
	}
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		return err
	}
	s := grpc.NewServer()
	pb.RegisterServicePostsServer(s, &Server{db: *db})
	log.Printf("gRPC server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		return err
	}
	return nil
}
