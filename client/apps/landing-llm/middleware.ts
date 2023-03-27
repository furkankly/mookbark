import { NextRequest, NextResponse } from "next/server";

import { getSession } from "./session";

export async function middleware(req: NextRequest) {
  if (process.env.NODE_ENV === "development") {
    // cors protocol requires options(preflight) to pass without any credentials
    if (req.method === "OPTIONS") {
      return new Response("Ok", {
        status: 200,
      });
    }
  }
  try {
    await getSession();
    return NextResponse.next();
  } catch (err) {
    console.error(err);
    return NextResponse.json({ error: "Not authenticated" }, { status: 401 });
  }
}

export const config = {
  matcher: ["/api/(chat|ingest)(.*)"],
};
