# Task App

# Setup

1. Copy env.example file in root folder to .env
  - cp env.example .env
2. Modify .env file according to your personal preference.
3. Create sqlite file with same name as in env file in storage folder.
  - touch storage/<db_file_name>
4. Migrate database.
  - sea-orm-cli migrate up
5. Run project.
  - cargo run
