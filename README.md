# TimeAway API

An api for submitting vacation requests.

### Dependencies

- Rust
	- Actix
	- MongoDB
	- serde
	- log
	- log4rs
	- bson
	- dotenv

### Folder structure

- timeaway_api
	- src
		- actix
			- routes.rs
			- mod.rs
		- mongo
			- db.rs
			- models.rs
			- mod.rs
	- main.rs
- Cargo.toml
- Readme.md
- log4rs.yaml
- .env
	
### Data DB and collection

- timeaway
	- requests

### Config file

An .env file is used for configuration details, the file goes in the root of the application.

```
# Actix server port
ACTIX_PORT=#### - NOTE: 8080 used if not defined

# Database connection
DATABASE_URL=mongodb://**THE MONGODB URL**/

# Database name
DATABASE_NAME=**THE DATABASE NAME**

# Database collection
DATABASE_COLLECTION=**THE COLLECTION NAME**
```

## Request Struct

```
GET 	"/api/requests"
GET 	"/api/requests/{id}"
POST 	"/api/requests"
PATCH   "/api/requests/{id}"
DELETE  "/api/requests/{id}"
```

```
cargo watch -x run
```

