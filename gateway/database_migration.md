cargo install sea-orm-cli@^2.0.0-rc

sea-orm-cli migrate init


sea-orm-cli migrate fresh

sea-orm-cli generate entity -u DATABASE_URL -o src/entities