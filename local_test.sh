#!/bin/sh

NAME=bendun.cc
PORT=80

docker build -t "$NAME" . && docker run -p "$PORT":8043 "$NAME"
