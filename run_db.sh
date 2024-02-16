docker stop some-postgres
docker rm some-postgres
docker run -p 5432:5432 --name some-postgres -e POSTGRES_PASSWORD=password -d postgres
