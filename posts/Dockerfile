FROM golang:1.22-alpine AS builder
WORKDIR /build
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -ldflags="-s -w" -o main src/main.go src/handlers.go


FROM alpine
RUN apk update --no-cache && apk add --no-cache ca-certificates
COPY --from=builder /build/main /app/main
CMD ["/app/main"]
