
include ../golang.mk

.PHONY: lambda

build: graphql.webapp

lambda: bootstrap.zip

CHANGELOG.md: ../../CHANGELOG.md
	@cp ../../CHANGELOG.md CHANGELOG.md

bootstrap: graphql.webapp
	@cp graphql.webapp bootstrap

bootstrap.zip: bootstrap
	@zip bootstrap.zip bootstrap
