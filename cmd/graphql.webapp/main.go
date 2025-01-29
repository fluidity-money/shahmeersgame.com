package main

//go:generate go run github.com/99designs/gqlgen generate

import (
	"log"
	"net/http"
	"database/sql"
	"os"

	_ "github.com/lib/pq"

	"github.com/aws/aws-lambda-go/lambda"

	"github.com/awslabs/aws-lambda-go-api-proxy/httpadapter"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/playground"

	"github.com/fluidity-money/shahmeersgame.com/cmd/graphql.webapp/graph"
)

const (
	// EnvDatabaseUri to connect to using Postgres.
	EnvDatabaseUri = "SPN_TIMESCALE"

	// EnvBackendType to use to listen the server with, (http|lambda).
	EnvBackendType = "SPN_LISTEN_BACKEND"

	// EnvListenAddr to listen the HTTP server on.
	EnvListenAddr = "SPN_LISTEN_ADDR"
)

func main() {
	db, err := sql.Open("postgres", os.Getenv(EnvDatabaseUri))
	if err != nil {
		log.Fatalf("open postgres: %v", err)
	}
	srv := handler.NewDefaultServer(graph.NewExecutableSchema(graph.Config{
		Resolvers: &graph.Resolver{db},
	}))
	http.Handle("/", srv)
	http.Handle("/playground", playground.Handler("GraphQL playground", "/query"))
	switch typ := os.Getenv(EnvBackendType); typ {
	case "lambda":
		lambda.Start(httpadapter.NewV2(http.DefaultServeMux).ProxyWithContext)
	case "http":
		err := http.ListenAndServe(os.Getenv(EnvListenAddr), nil)
		log.Fatalf(
			"err listening, %#v not set?: %v",
			EnvListenAddr,
			err,
		)
	default:
		log.Fatalf(
			"unexpected listen type: %#v, use either (lambda|http) for SPN_LISTEN_BACKEND",
			typ,
		)
	}
}
