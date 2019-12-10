# Docker Guide

This repository has a `Dockerfile` in the root which builds an image with the
`lighthouse` binary installed.

To use the image, first build it (this will likely take several minutes):

```bash
$ docker build . -t lighthouse
```

Once it's built, run it with:

```bash
$ docker run lighthouse lighthouse --help
```

_Note: the first `lighthouse` is the name of the tag we created earlier. The
second `lighthouse` refers to the binary installed in the image._

To start a beacon node with a persistent data directory and access to the JSON-RPC API on the host at port 5052 run the image with:

```bash
$ docker run --name=lighthouse -v $HOME/lighthouse-data:/root/.lighthouse -p 127.0.0.1:5052:5052 lighthouse lighthouse beacon_node --http-address=0.0.0.0 --http-port=5052 --http
```
