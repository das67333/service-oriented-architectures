//go:build service_tests
// +build service_tests

package main

import (
	"context"
	"fmt"
	"log"
	"os"
	pb "service-posts/protos"
	"service-posts/server"
	"testing"
	"time"

	"github.com/docker/go-connections/nat"
	"github.com/jmoiron/sqlx"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/modules/postgres"
	"github.com/testcontainers/testcontainers-go/wait"
)

var (
	ctx         = context.Background()
	db          *sqlx.DB
	pgContainer testcontainers.Container
)

func setupDbContainer() error {
	var err error
	var pgHost string
	var pgPort nat.Port
	var pgPassword = "pg_password"

	pgContainer, err = postgres.RunContainer(ctx,
		testcontainers.WithImage("postgres:16-alpine"),
		postgres.WithPassword(pgPassword),
		postgres.WithInitScripts("init.sql"),
		testcontainers.WithWaitStrategy(
			wait.ForLog("database system is ready to accept connections").
				WithOccurrence(2).WithStartupTimeout(5*time.Second)),
	)
	if err != nil {
		return err
	}

	pgHost, err = pgContainer.Host(ctx)
	if err != nil {
		return err
	}
	pgPort, err = pgContainer.MappedPort(ctx, "5432")
	if err != nil {
		return err
	}
	dsn := fmt.Sprintf(
		"user=postgres password=%s host=%s port=%s sslmode=disable",
		pgPassword, pgHost, pgPort.Port(),
	)
	db, err = sqlx.Connect("postgres", dsn)
	if err != nil {
		return err
	}
	initDb(db)
	return nil
}

func TestMain(m *testing.M) {
	if err := setupDbContainer(); err != nil {
		log.Fatal(err)
	}

	exitCode := m.Run()
	if err := pgContainer.Terminate(ctx); err != nil {
		log.Fatalf("failed to terminate container: %s", err)
	}
	os.Exit(exitCode)
}

func TestCreatePost(t *testing.T) {
	s := server.Server{Db: *db}
	id, err := s.CreatePost(ctx, &pb.RequestCreate{
		Login:   "test_login",
		Content: "test_content",
	})
	if err != nil {
		t.Fatal(err)
	}
	var post server.Post
	err = s.Db.GetContext(ctx, &post, "SELECT * FROM posts WHERE id = $1", id.Value)
	if err != nil {
		t.Fatal(err)
	}
	if post.Login != "test_login" {
		t.Fatalf("expected login: test_login, got: %s", post.Login)
	}
	if post.Content != "test_content" {
		t.Fatalf("expected content: test_content, got: %s", post.Content)
	}
	db.MustExecContext(ctx, "TRUNCATE TABLE posts")
}
