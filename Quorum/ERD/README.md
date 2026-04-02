# Entity Relationship Diagram
This directory contains the Entity Relationship Diagram (ERD) for this project.
The ERD visually represents the relationships between different entities in the database, helping to understand the structure and design of the database schema.

It's advised to use this ERD as a reference when working with the database, knowing how to access something and how an entity relates to another.

THE ERD is an SVG image that can be opened in most web browsers and image viewers.

## Generating the ERD
The ERD is automatically created when writing the database initial.surql.

https://app.surrealdb.com/ Ensure that docker is running (see docker folder) have the surrealdb connected to the running docker database and past the initial.surql into it.

Run the query to create all the tables and relationships if the docker db doesn't already have that. THen in the left side bar, head to designer. This shows an ERD basically the exact same as the queries written.
By viewing it this way, this shows more detailed information about each time and their relationships. 

### Export
To export the ERD, follow the above steps, once on the table graph view, `right click` anywhere on the graph and select `Export as SVG`. There is a PNG option but I this didn't work when I tried it.