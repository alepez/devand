#!/bin/bash

set -e

mkdir -p static

wget https://unpkg.com/purecss@2.0.3/build/pure-min.css -O static/pure.css
wget https://unpkg.com/purecss@2.0.3/build/grids-responsive-min.css -O static/grids-responsive.css
