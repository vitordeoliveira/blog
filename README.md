# blog

Personal Blog

It contains a dockerfile and a docker-compose file
the dockerfile has some arguments, being relevant the SQLITE_DB
for the runtime it is important to point to a valid .sqlite file

use a volume if necessary

for publish

docker build -t /image tag/ /account_tag/:/version/ PATH_DOCKERFILE
docker push /account_tag/:/version/

## cargo utils

cargo login --registry invisible-matrix
