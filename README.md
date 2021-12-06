# GRPC elevation service based on gdal data

Relevation expose a GRPC interface to retrieve elevation data from geo points from GDAL tiles.

# TODO

- [ ] LRU in memory cache
- [ ] Authentication
- [ ] HTTP api over grpc
- [ ] Tests
- [ ] Documentation

# Usage

**Under active development**

## Run server

```
relevation run [path to toml config]
```

## Example config

```toml
host = "127.0.0.1"
port = 50051

[[sources]]
id = "some-source-id"
name = "Some Source"
link = "http://link-to-source-data.tld"
attributions = '''
Some authors attributions if needed
'''
resolution = 30
path = "/path/to/root/folder/some-source-folder"
```

## Log level

Set the env variable `RELEVATION_LOG` to `debug`, `info`, `warn`, ...

## Test server

```sh
grpcurl -plaintext -import-path ./proto -proto ./proto/relevation.proto -d '' [::]:50051 relevation.Relevation/Ping
grpcurl -plaintext -import-path ./proto -proto relevation.proto -d '{"point": { "lat": 0, "lng": 0 }}' [::]:50051 relevation.Relevation/GetElevation

# Query for Paris
grpcurl -plaintext -import-path ./proto -proto relevation.proto -d '{"point": { "lat": 48.864716, "lng": 2.349014 }}' [::]:50051 relevation.Relevation/GetElevation
```

## Split geotiff data

```sh
./scripts/create-tiles.sh source.tif 10 10
```

# LICENSE

See `LICENSE`.
