FROM rust AS rust-builder
COPY ./projects/ /projects/
WORKDIR /projects
RUN cargo update
RUN cargo run > projects.html

FROM ubuntu:22.04 AS builder
RUN apt update && apt install -y sqlite3 python3 python3-pip graphviz git
COPY . /site/
RUN cd /site/; git submodule update --init --remote --depth 1
RUN cd /site/best-ratings-in-given-year && ./generate.sh > /site/public/best-ratings-in-given-year.html
RUN cd /site/all-conlang/; pip3 install -r requirements.txt; python3 compile.py
RUN cp /site/all-conlang/bee_movie_quote.jpg /site/all-conlang/*.png /site/all-conlang/all.html /site/public/

FROM pierrezemb/gostatic
COPY --from=rust-builder /projects/projects.html /srv/http/
COPY --from=builder /site/public/ /srv/http/
