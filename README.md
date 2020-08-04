# rust-server-playground

## How to deploy this in docker
### Build:
````
> docker build -t rust-server-playground .
````
### Run:
````
> docker run  --name rust-play -d -p 3030:3030 rust-server-playground
````
### Run with a CPU limit:
````
> docker run --name rust-play -d -p 3030:3030 —cpus=“2.0” rust-server-playground
````
### Stop:
````
> docker stop rust-play 
> docker rm rust-play
````
## Benchmark Performance:
````
> cargo install drill
> drill --benchmark benchmark.yml --stats
````