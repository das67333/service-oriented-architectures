package server

import (
	"testing"
	"time"

	"github.com/jmoiron/sqlx"
)

func TestRunGrpcServerNil(t *testing.T) {
	t.Run("runGrpcServer(nil)", func(t *testing.T) {
		err := RunGrpcServer(nil)
		if err == nil {
			t.Errorf("no error returned; want error")
		}
	})

}

func TestRunGrpcServerDefault(t *testing.T) {

	t.Run("runGrpcServer(default)", func(t *testing.T) {
		done := make(chan bool)

		go func() {
			if err := RunGrpcServer(&sqlx.DB{}); err != nil {
				t.Errorf("RunGrpcServer() returned error: %v", err)
			}
			done <- true
		}()

		select {
		case <-time.After(1 * time.Second):
		case <-done:
			t.Fatal("TestRunGrpcServer stopped unexpectedly")
		}
	})
}
