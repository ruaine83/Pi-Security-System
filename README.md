# Pi Security System
Security System based around RaspberryPi and ESP8266 Boards

## Overview

### Desired Featuers
Some features that will be included in the project

1. Alerts types
  - Audible Alarm
  - Visual Alarm on the various TFT/LCD screens connected to _slave_ nodes
  - SMS to a list of users
  - SMS to include pictures of visually-monitored locations
2. Customization via Web interface
3. Touch Screen interface for _master_.
4. 

## Hardware
### RaspberryPi X
The RaspberyPi (Rpi or rpi from here) system will be used as the _master_ node. It will contain the main program that will receive notificiation from the _slave_ nodes, save a record, take/save still images. The RPi will be attached to a touch-screen interface, the WiFi network used by the _slave_ nodes, and a wired ethernet connection to the wider world.

### ESP8266 Boards
ESP8266-based boards (ESP) will operate as the various _slave_ nodes, and will monitor various sensors attached to them. They will interface with a private WiFi network that they will utilize to communicate with the _master_ node as well as each other as necessary. They will also have small (1-2", likely monochrome) LCD screens attached to them in order to show alert and status messages at-a-glance.

## Licensing
Please see the license file for more information. Feel free to use this code as you see fit. Creator is not responsible in any way for any results you may experience using this software. Use at your own risk.
