import { boolean, pgTable, varchar, primaryKey } from "drizzle-orm/pg-core";

export const bookmark = pgTable(
  "bookmark",
  {
    url: varchar("url", { length: 255 }).notNull(),
    userId: varchar("user_id", { length: 255 }).notNull(),
    ingested: boolean("ingested").default(false),
  },
  (table) => {
    return { pk: primaryKey({ columns: [table.url, table.userId] }) };
  }
);
