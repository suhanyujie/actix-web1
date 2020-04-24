


## dev
- `systemfd --no-pid -s http::3000 -- cargo watch -x run`
- Mac 上 kill 旧的进程 `ps -A | grep actix-web | awk '{print $1}' | xargs kill`

### db
- `docker run --name mongo1 -d -p 27017:27017 -v ~/www2/software/docker/containerData/rs_article/db:/data/db mongo:4.2.5`

## 参考资料
- https://github.com/nintha/demo-myblog
