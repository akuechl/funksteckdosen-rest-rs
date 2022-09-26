# funksteckdosen-rest-rs

A little rest server to use radio-controlled sockets with wiringPi in a Raspberry PI.

This program provides a HTTP server to call radio-controlled sockets. This program use the [funksteckdose crate](https://crates.io/crates/funksteckdose) which use the [wiringpi crate](https://crates.io/crates/wiringpi). You have to install [wiringPi](http://wiringpi.com/) library on your system.

## Install WiringPi

Please check [here](http://wiringpi.com/download-and-install/). Maybe wiringPi is pre-installed.

## Compile

At the moment a used library needs the nightly rust compiler.
````
rustup override set nightly
````

## Download

Several downloads can be found on the [release page](https://github.com/akuechl/funksteckdosen-rest-rs/releases).

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

## Run as service

* change path in funksteckdosen_rest_rs.service

* cp file to systemd
````
sudo cp funksteckdosen_rest_rs.service /lib/systemd/system/
````

* commands
````
# start
sudo systemctl start funksteckdosen-rest-rs.service

# stop
sudo systemctl stop funksteckdosen-rest-rs.service

# show status
sudo systemctl status funksteckdosen-rest-rs.service
````

* activate after reboot
````
sudo systemctl enable funksteckdosen-rest-rs.service 
````

* deactivate after reboot
````
sudo systemctl disable funksteckdosen-rest-rs.service 
````

## Use Case

I control the sockets with my [Home Assistant]() instance. For this I use the [rest comand](https://www.home-assistant.io/integrations/rest_command/).

````
# configuration.yaml
#
rest_command:
        socket_2:
                url: http://192.168.178.123:12345/pin/11100/2/0

# automations.yaml
# light_living_room is a input_button from helpers
#
- id: 'socket_2_auto'
        alias: Light Living Room
        description: Red Lamp
        trigger:
        - platform: state
          entity_id:
                - input_button.light_living_room
        condition: []
        action:
        - service: rest_command.socket_2
          data: {}
        mode: single
````
