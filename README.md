


## dev
- `systemfd --no-pid -s http::3000 -- cargo watch -x run`

### db
- `docker run --name mongo1 -d -p 27017:27017 -v ~/www2/software/docker/containerData/rs_article/db:/data/db mongo:4.2.5`

## 参考资料
- https://github.com/nintha/demo-myblog