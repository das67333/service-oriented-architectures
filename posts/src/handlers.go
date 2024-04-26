package main

import (
	"context"
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
	err := s.db.QueryRow(`
	INSERT INTO posts (login, created_at, content)
	VALUES ($1, NOW(), $2)
	RETURNING id`,
		data.Login,
		data.Content).Scan(&id)
	if err != nil {
		return nil, err
	}
	return &pb.PostId{Id: uint64(id)}, err
}

func (s *Server) UpdatePost(ctx context.Context, data *pb.RequestUpdate) (*pb.ReturnCode, error) {
	var storedLogin string
	err := s.db.Get(&storedLogin, "SELECT login FROM posts WHERE id = $1", data.Id)
	if err != nil {
		return &pb.ReturnCode{Code: pb.Status_PostNotFound}, nil
	}
	if storedLogin != data.Login {
		return &pb.ReturnCode{Code: pb.Status_LoginMismatch}, nil
	}
	_, err = s.db.Exec("UPDATE posts SET content = $1 WHERE id = $2", data.Content, data.Id)
	if err != nil {
		return nil, err
	}
	return &pb.ReturnCode{Code: pb.Status_Ok}, nil
}

func (s *Server) RemovePost(ctx context.Context, data *pb.RequestRemove) (*pb.ReturnCode, error) {
	var storedLogin string
	err := s.db.Get(&storedLogin, "SELECT login FROM posts WHERE id = $1", data.Id)
	if err != nil {
		return &pb.ReturnCode{Code: pb.Status_PostNotFound}, nil
	}
	if storedLogin != data.Login {
		return &pb.ReturnCode{Code: pb.Status_LoginMismatch}, nil
	}
	_, err = s.db.Exec("DELETE FROM posts WHERE id = $1", data.Id)
	if err != nil {
		return nil, err
	}
	return &pb.ReturnCode{Code: pb.Status_Ok}, nil
}

func (s *Server) GetPost(ctx context.Context, data *pb.RequestGetOne) (*pb.OptionalPost, error) {
	var post Post
	err := s.db.Get(&post, "SELECT * FROM posts WHERE id = $1", data.Id)
	if err != nil {
		return &pb.OptionalPost{Code: pb.Status_PostNotFound}, nil
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
	var any_id uint64
	err := s.db.QueryRow("SELECT id FROM posts WHERE login = $1 LIMIT 1", data.Login).Scan(&any_id)
	if err != nil {
		return &pb.Posts{Code: pb.Status_UserNotFound}, nil
	}
	var posts []*Post
	err = s.db.Select(&posts, `
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
