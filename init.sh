#!/bin/bash

# Wait for ScyllaDB to be ready
until cqlsh scylladb 9042 -e "describe cluster"
do
    echo "Waiting for ScyllaDB to be up..."
    sleep 2
done

# Run the schema script
cqlsh scylladb 9042 -f /init.cql
