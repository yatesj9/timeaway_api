# TimeAway API

An api for submitting vacation requests.

### Dependencies

- Rust
	- Actix
	- MongoDB
	- serde
	- log
	- log4rs

### Folder structure

- timeaway_api
	- src
		- actix
			- routes.rs
			- mod.rs
		- mongo
			- db.rs
			- mod.rs
	- main.rs
- Cargo.toml
- Readme.md
- log4rs.yaml
- .env
	
### Data

- timeaway
	- requests

### Config file

An .env file is used for configuration details, the file goes in the root of the application.

```
# Database connection
DATABASE_URL=mongodb://**THE MONGODB URL**/

# Database name
DATABASE_NAME=**THE DATABASE NAME**

# Database collection
DATABASE_COLLECTION=**THE COLLECTION NAME**
```

## Request Struct

TODO



```
cargo watch -x run
```

