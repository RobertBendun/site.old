FROM ubuntu:22.04 AS generic_builder

RUN apt update && apt install -y sqlite3
COPY . /site/
RUN cd /site/best-ratings-in-given-year && ./generate.sh > /site/public/best-ratings-in-given-year.html

FROM node AS tiddlywiki

RUN npm i -g tiddlywiki
COPY wiki /wiki/
RUN tiddlywiki /wiki --rendertiddlers '[!is[system]]' $:/core/templates/static.tiddler.html static text/plain

FROM pierrezemb/gostatic
COPY --from=generic_builder /site/public/ /srv/http/
COPY --from=tiddlywiki /wiki/output/static/*.html /wiki/static.css /srv/http/wiki/
