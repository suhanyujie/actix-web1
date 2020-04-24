ps -A | grep actix-web | awk '{print $1}' | xargs kill
systemfd --no-pid -s http::3000 -- cargo watch -x run
