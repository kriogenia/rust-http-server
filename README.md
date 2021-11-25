# rust-http-server
HTTP Server developed with Rust.

This one is a little project made to address two objectives.
The first one is in line with my current status as web developer, to develop my own HTTP Server and 
get a deeper learning about the infrastructure of those.
And the other one is to learn Rust, a language I've been eager to learn. 
And damn, it really was a ride.

## Features

This server can:

* Easily route as many new endpoints as you wish.
* Work with the different `METHOD` types.
* Build different types of Responses.
* Handle and map query parameters.
* Serve static pages and resources.
* Accept different and customizable request handlers.
* Manage gracefully all known errors.
* Evade Directory Traversal Attacks

## Planned

I still to plan to add some additional features:

* A swappable handler to answer to the endpoints with JSON.
* Headers to the responses.
* An endpoint with logic based on the queries.
* More environment properties.

## Deployment

To be able to deploy this server you'll need to have Rust installed on your system.
Once it's installed you can clone the repo and use the following command to execute it in the port 8080.

```shell
rustc run --package http-server --bin http-server
```

It's also possible to use the following environment variables to customize the server:

* PUBLIC_PATH: to establish the path to use as public assets route

## Usage

Once the server is deployed it can be tested calling the different endpoints, for example:

```shell
curl -XGET "127.0.0.1:8080/hello"
```

Or just navigating to [http://127.0.0.1:8080/](http://127.0.0.1:8080/) to see the index page.
Going any other endpoint will return the 404 page.