#!/bin/bash

mdbook watch &
trap 'kill $(jobs -p)' EXIT

# this is an npm http server that will hot reload 
# a static html page if it is changed
cd book && live-server