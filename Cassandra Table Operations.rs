# Run the below command on the newly opened terminal
cqlsh --username cassandra --password PASSWORD

# Create Keyspace
CREATE KEYSPACE training
WITH replication = {'class':'SimpleStrategy', 'replication_factor' : 3};

#  Create a table
use training;
CREATE TABLE movies(
movie_id int PRIMARY KEY,
movie_name text,
year_of_release int
);

# Verify that the table got created by listing all tables
describe tables;

# Describe a table
describe movies

# Alter a table
ALTER TABLE movies
ADD genre text;

# Verify the changes using the below command
describe movies;

# Drop a table
drop table movies;

# Verify using the below command. You should get an error.
describe movies;

# 