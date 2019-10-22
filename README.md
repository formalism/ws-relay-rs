* ws-relay-rs

This program simply relays data between WebSocket and TCP servers. Data received from WebSocket clients
is forwarded to the specified TCP server, and vice-versa. It resembles [ws-tcp-relay](https://github.com/isobit/ws-tcp-relay)
albeit this program is written in Rust language and without TLS support.

* Usage

<pre>
Usage: ws-relay-rs <options>
    -p | -P int     port of the TCP server
    -s | -S adr     address of the TCP server
    -w | -W int     port to wait on the WebSocket client 
</pre>

sample directory contains example TCP server application also written in Rust,
which generates sample image data.
index.html is a sample HTML file which connects to this program, then transfer
image data via WebSocket.

* Build

<pre>
cargo build --release
</pre>

Sample program is also built with the above command.

