#!/bin/bash

cargo run &
cd electron && yarn start &
cd ui && yarn start &


wait
echo "All 3 complete"