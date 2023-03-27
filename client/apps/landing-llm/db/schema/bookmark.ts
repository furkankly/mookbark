import {
  boolean,
  mysqlTable,
  varchar,
  primaryKey,
} from "drizzle-orm/mysql-core";

export const bookmark = mysqlTable(
  "bookmark",
  {
    url: varchar("url", { length: 255 }).notNull(),
    userId: varchar("user_id", { length: 255 }).notNull(),
    ingested: boolean("ingested").default(false),
  },
  (table) => {
    return { pk: primaryKey({ columns: [table.url, table.userId] }) };
  },
);
