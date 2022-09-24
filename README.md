# funksteckdosen-rest-rs

A little rest server to use radio-controlled sockets with wiringPi in a Raspberry PI.

This program provides a HTTP server to call radio-controlled sockets. This program use the [funksteckdose crate](https://crates.io/crates/funksteckdose) which use the [wiringpi crate](https://crates.io/crates/wiringpi). You have to install [wiringPi](http://wiringpi.com/) library on your system.

## Install WiringPi

Please check [here](http://wiringpi.com/download-and-install/). Maybe wiringPi is pre-installed.

## Starting the server

Starts the server on port 12345 bind to IP 127.0.0.1 (default):
````
./funksteckdosen-rest-rs --port 12345
````

Starts the server on port 12345 without any IP binding:
````
./funksteckdosen-rest-rs --port 12345 --bind 0.0.0.0
````

## Switch the socket

````
http://localhost:12345/pin/11100/3/0
````
* 11100 is the system code

* 3 is the unit code

* 0 is the command (off)
