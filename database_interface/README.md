Initialize docker container:
```bash
docker run --name test_bed -p 3306:3306 -e MYSQL_ROOT_PASSWORD=root -e MYSQL_USER=test_user -e MYSQL_PASSWORD=test_password -d mysql:latest
```