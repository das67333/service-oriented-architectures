package server

import (
	"fmt"
	"log"
	"net"
	pb "service-posts/protos"

	"github.com/jmoiron/sqlx"
	"google.golang.org/grpc"
)

func TryCreateTable(db *sqlx.DB) {
	db.MustExec(`
	CREATE TABLE IF NOT EXISTS posts (
		login VARCHAR,
		id SERIAL PRIMARY KEY,
		created_at TIMESTAMP,
		content VARCHAR
	)`)
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
