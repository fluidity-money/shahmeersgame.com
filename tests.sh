#!/bin/sh -e

cargo test

cargo mutants
