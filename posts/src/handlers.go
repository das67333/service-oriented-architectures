package main

import (
	"context"
	"database/sql"
	pb "service-posts/protos"
	"time"

	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
	"google.golang.org/protobuf/types/known/timestamppb"
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

type Post struct {
	Login     string    `db:"login"`
	Id        uint64    `db:"id"`
	CreatedAt time.Time `db:"created_at"`
	Content   string    `db:"content"`
}

func (s *Server) CreatePost(ctx context.Context, data *pb.RequestCreate) (*pb.PostId, error) {
	var id uint64
	err := s.db.GetContext(ctx, &id, `
	INSERT INTO posts (login, created_at, content)
	VALUES ($1, NOW(), $2)
	RETURNING id`,
		data.Login,
		data.Content)
	if err != nil {
		return nil, err
	}
	return &pb.PostId{Id: uint64(id)}, nil
}

func (s *Server) UpdatePost(ctx context.Context, data *pb.RequestUpdate) (*pb.ReturnCode, error) {
	tx, err := s.db.BeginTxx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	var storedLogin string
	err = tx.GetContext(ctx, &storedLogin, "SELECT login FROM posts WHERE id = $1", data.Id)
	if err != nil {
		if err == sql.ErrNoRows {
			return &pb.ReturnCode{Code: pb.Status_PostNotFound}, nil
		}
		return nil, err
	}
	if storedLogin != data.Login {
		return &pb.ReturnCode{Code: pb.Status_LoginMismatch}, nil
	}
	_, err = tx.ExecContext(ctx, "UPDATE posts SET content = $1 WHERE id = $2", data.Content, data.Id)
	if err != nil {
		return nil, err
	}
	if err = tx.Commit(); err != nil {
		return nil, err
	}
	return &pb.ReturnCode{Code: pb.Status_Ok}, nil
}

func (s *Server) RemovePost(ctx context.Context, data *pb.RequestRemove) (*pb.ReturnCode, error) {
	tx, err := s.db.BeginTxx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	var storedLogin string
	err = tx.GetContext(ctx, &storedLogin, "SELECT login FROM posts WHERE id = $1", data.Id)
	if err != nil {
		if err == sql.ErrNoRows {
			return &pb.ReturnCode{Code: pb.Status_PostNotFound}, nil
		}
		return nil, err
	}
	if storedLogin != data.Login {
		return &pb.ReturnCode{Code: pb.Status_LoginMismatch}, nil
	}
	if _, err = tx.ExecContext(ctx, "DELETE FROM posts WHERE id = $1", data.Id); err != nil {
		return nil, err
	}
	if err = tx.Commit(); err != nil {
		return nil, err
	}
	return &pb.ReturnCode{Code: pb.Status_Ok}, nil
}

func (s *Server) GetPost(ctx context.Context, data *pb.RequestGetOne) (*pb.OptionalPost, error) {
	var post Post
	err := s.db.GetContext(ctx, &post, "SELECT * FROM posts WHERE id = $1", data.Id)
	if err != nil {
		if err == sql.ErrNoRows {
			return &pb.OptionalPost{Code: pb.Status_PostNotFound}, nil
		}
		return nil, err
	}
	pb_post := pb.Post{
		Login:     post.Login,
		Id:        post.Id,
		CreatedAt: timestamppb.New(post.CreatedAt),
		Content:   post.Content,
	}
	return &pb.OptionalPost{Code: pb.Status_Ok, Post: &pb_post}, nil
}

func (s *Server) GetPosts(ctx context.Context, data *pb.RequestGetMany) (*pb.Posts, error) {
	tx, err := s.db.BeginTxx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	var any_id uint64
	err = tx.GetContext(ctx, &any_id, "SELECT id FROM posts WHERE login = $1 LIMIT 1", data.Login)
	if err != nil {
		return &pb.Posts{Code: pb.Status_UserNotFound}, nil
	}
	var posts []*Post
	err = tx.SelectContext(ctx, &posts, `
	SELECT *
	FROM posts
	WHERE id >= $1 AND login = $2
	ORDER BY id
	LIMIT $3`,
		data.StartId,
		data.Login,
		data.Count,
	)
	if err != nil {
		return nil, err
	}
	if err = tx.Commit(); err != nil {
		return nil, err
	}

	pb_posts := make([]*pb.Post, len(posts))
	for i, post := range posts {
		pb_posts[i] = &pb.Post{
			Login:     post.Login,
			Id:        post.Id,
			CreatedAt: timestamppb.New(post.CreatedAt),
			Content:   post.Content,
		}
	}
	return &pb.Posts{Code: pb.Status_Ok, Posts: pb_posts}, nil
}
