# Questions api profile

``` bash
sudo chmod -R o+w target
```

Compilar la aplicación:

``` bash
docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder

cargo build --release
```


``` bash
docker build --no-cache --pull --platform linux/arm64 -f ./Dockerfile -t kennycallado/uv-questions_api-profiles:v0.1-arm64 .
```

``` bash
docker build --no-cache --pull --platform linux/amd64 -f ./Dockerfile -t kennycallado/uv-questions_api-profiles:v0.1-amd64 .
```

``` bash
docker push -a kennycallado/uv-questions_api-profiles
```

## TODO:

- [x] add migrations on bootstrap
- [ ] A way to populate the database from a distribuited database...

