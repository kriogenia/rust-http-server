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
* Support nested and dedicated handlers/routing.
* Implement REST and work with the different `METHOD` types.
* Build different types of Responses.
* Handle, map and compute query parameters.
* Serve static pages and resources.
* Accept different and customizable request handlers.
* Manage gracefully all known errors.
* Evade Directory Traversal Attacks.

## Planned

I still to plan to add some additional features:

* Headers to the responses.
* More environment properties.
* *Doubtful*: Make the server multithreaded

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
curl -XGET "http://127.0.0.1:8080/hello"
```

Or just navigating to [http://127.0.0.1:8080/](http://127.0.0.1:8080/) to see the index page.
Going any other endpoint in your browser will yield the 404 page.

Aside from that, the server also has an API REST with its own and more powerful Hello World!,
because it accepts a custom name to generate the salute using query parameters.

```shell
curl -XGET "http://127.0.0.1:8080/api/hello"
curl -XGET "http://127.0.0.1:8080/api/hello?name=Kaladin"
```

And finally, a complete and working REST interface using different methods to handle a counter.
You can try the following requests to test it.

```shell
curl -XGET "http://127.0.0.1:8080/api/count"
curl -XPOST "http://127.0.0.1:8080/api/count"
curl -XPOST "http://127.0.0.1:8080/api/count?value=3"
curl -XDELETE "http://127.0.0.1:8080/api/count"
curl -XPOST "http://127.0.0.1:8080/api/count?value=NaN"
```

## Design

The follow diagram represents the architecture and design of the server.
The overall flow starts with the creation of a new Server with the address it will be deployed on.
That server can be launch providing it a request Handler.

In this basic server the Handler are also the Routers, but that logic could be easily extracted.
There's also a MultiHandler using the composite pattern to allow the Server to work with more than one handler.
In this prototype we also created a WebHandler and an ApiHandler, the first one serves the html pages
and the other one responds to API REST requests returning JSON responses.
Both Handlers use a FileReader that reads and returns the content of the requested public file.
It also provides protection against Directory Traversal Attacks.

Each request comes from a TcpListener of the Server that it's parsed into a Request.
Requests contain the method, protocol, headers and query parameters of the request.
The entities implementing the Handler trait receive those requests and generate a Response
if they how to satisfy the Request.
Those Responses are then returned to the Server and from there to the client.

### TODO CLASS DIAGRAM