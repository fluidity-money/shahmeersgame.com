#!/bin/sh -e

table="shahmeersgame_migrations"

dbmate -d migrations --migrations-table "$table" -u "$1" up
