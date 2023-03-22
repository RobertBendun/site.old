FROM ubuntu:22.04 AS builder

RUN apt update && apt install -y sqlite3
COPY . /site/
RUN cd /site/best-ratings-in-given-year && ./generate.sh > /site/public/best-ratings-in-given-year.html

FROM pierrezemb/gostatic
COPY --from=builder /site/public/ /srv/http/
