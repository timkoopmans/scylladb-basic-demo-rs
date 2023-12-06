CREATE KEYSPACE demo
    WITH replication = {'class': 'NetworkTopologyStrategy', 'replication_factor': 1};

CREATE TABLE demo.users (
   user_id uuid PRIMARY KEY,
   first_name text,
   last_name text,
   age int
);