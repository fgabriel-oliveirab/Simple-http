# Simple-http
## About
A simple HTTP static files server written in Rust from scratch without frameworks. 
I just used async_std::net to handle tcp streams, async_std::io to read files 
and used the async_std async runtime.

## How to run
### Requirements:
> Rust compiler with cargo.

It runs at 127.0.0.1:8080 by default. I haven't added a way to customize it without changing the code yet.

## Features
By now, it just returns static html files to GET requests at / and /foss and returns a 404 error html to anything else.
