set -ex

wasm-pack build --target web --out-dir www/pkg

echo "Now serving on http://127.0.0.1:8080/html"
http -a 127.0.0.1 -p 8080 www

