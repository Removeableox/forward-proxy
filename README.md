# rust forward proxy (WIP)

My current (WIP) final project that encapsulates various modules I'm working on with the goal of exploring server programming with only the rust standard libraries.

Sub projects I'm writing to make this:
[x] http-client (just need to be able to send http requests and read their responses)
[] https-client (largest one, manual implementation of the tls protocol)

External supplementary projects:
[x] http-server (I just have it returning a simple html page, but I got to play around with some multithreading)
[] https-server (not sure how to do this yet but I assume it'll be some sort of refactor of the https client/tls implementation)

Possible future projects:
[] reverse-proxy 
