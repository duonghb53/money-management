# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


## DB Diagram
![image](./resources/Money%20Management.png)

## DB Query create
```sql
CREATE TABLE "users" (
  "id" INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  "role_id" int UNIQUE NOT NULL,
  "full_name" varchar,
  "phone_number" varchar,
  "email" varchar,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "role" (
  "id" INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  "name" varchar,
  "continent_name" varchar,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "category" (
  "id" INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  "name" varchar,
  "type" int,
  "created_by" int UNIQUE NOT NULL,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "item" (
  "id" INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  "invoice_id" int NOT NULL,
  "category_id" int,
  "name" varchar,
  "amout" bigint,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "invoice" (
  "id" INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  "created_by" int,
  "name" varchar,
  "amount" bigint,
  "paymeny_method_id" int,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "paymeny_method" (
  "id" INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  "name" varchar,
  "created_by" int,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "investment" (
  "id" INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  "category_id" int,
  "name" varchar,
  "created_by" int,
  "created_at" timestamp,
  "updated_at" timestamp
);

ALTER TABLE "users" ADD FOREIGN KEY ("role_id") REFERENCES "role" ("id");

ALTER TABLE "category" ADD FOREIGN KEY ("created_by") REFERENCES "users" ("id");

ALTER TABLE "item" ADD FOREIGN KEY ("category_id") REFERENCES "category" ("id");

ALTER TABLE "item" ADD FOREIGN KEY ("category_id") REFERENCES "invoice" ("id");

ALTER TABLE "invoice" ADD FOREIGN KEY ("created_by") REFERENCES "users" ("id");

ALTER TABLE "paymeny_method" ADD FOREIGN KEY ("created_by") REFERENCES "users" ("id");

ALTER TABLE "investment" ADD FOREIGN KEY ("created_by") REFERENCES "users" ("id");

ALTER TABLE "investment" ADD FOREIGN KEY ("category_id") REFERENCES "category" ("id");

```
