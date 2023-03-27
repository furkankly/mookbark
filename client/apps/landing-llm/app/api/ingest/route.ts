import { NextRequest, NextResponse } from "next/server";

import { TogetherAIEmbeddings } from "@langchain/community/embeddings/togetherai";
import { Document } from "@langchain/core/documents";
import { PineconeStore } from "@langchain/pinecone";
import { Pinecone } from "@pinecone-database/pinecone";
import { RecursiveCharacterTextSplitter } from "langchain/text_splitter";
// import chromium from "@sparticuz/chromium-min";
// import puppeteer from "puppeteer-core";

import {
  // evaluateScrape,
  evaluateStaticScrape,
} from "@/app/api/ingest/evaluateScrape";
import { getSession } from "@/session";
import { db } from "@/db";
import { bookmark } from "@/db/schema/bookmark";
import { and, eq } from "drizzle-orm";

const pinecone = new Pinecone({
  apiKey: process.env.PINECONE_API_KEY ?? "",
});

export async function POST(req: NextRequest) {
  try {
    const bookmarkUrl = req.nextUrl.searchParams.get("bookmark_url");

    if (bookmarkUrl == null) {
      return NextResponse.json(
        { error: "No bookmark url found" },
        { status: 400 }
      );
    }

    // Switching to static scraping because of vercel hobby plan duration limit on serverless funcs

    // const browser = await puppeteer.launch({
    //   headless: true,
    //   ignoreDefaultArgs: ["--disable-extensions"],
    //   args: [...chromium.args, "--hide-scrollbars", "--disable-web-security"],
    //   defaultViewport: chromium.defaultViewport,
    //   executablePath: await chromium.executablePath(
    //     `https://github.com/Sparticuz/chromium/releases/download/v122.0.0/chromium-v122.0.0-pack.tar`
    //   ),
    // });
    // const page = await browser.newPage();
    // await page.goto(bookmarkUrl, {
    //   timeout: 180000,
    //   waitUntil: "load",
    // });
    // const text = await evaluateScrape(page);
    // await browser.close();

    const response = await fetch(bookmarkUrl, {
      signal: AbortSignal.timeout(8000),
    });
    const html = await response.text();
    const text = evaluateStaticScrape(html);

    const metadata = { source: bookmarkUrl };
    let docs = [new Document({ pageContent: text, metadata })];
    docs = docs.map((doc) => {
      return {
        ...doc,
      };
    });

    const textSplitter = new RecursiveCharacterTextSplitter({
      chunkSize: 1000,
      chunkOverlap: 200,
    });
    const splitDocs = await textSplitter.splitDocuments(docs);

    const embeddings = new TogetherAIEmbeddings({
      modelName: "togethercomputer/m2-bert-80M-8k-retrieval",
    });

    const PINECONE_INDEX_NAME = process.env.PINECONE_INDEX_NAME ?? "";
    const index = pinecone.index(PINECONE_INDEX_NAME);
    await PineconeStore.fromDocuments(splitDocs, embeddings, {
      pineconeIndex: index,
      namespace: bookmarkUrl,
    });

    const userId = await getSession();
    await db
      .update(bookmark)
      .set({ ingested: true })
      .where(and(eq(bookmark.userId, userId), eq(bookmark.url, bookmarkUrl)));
  } catch (err: any) {
    console.error(err);
    return NextResponse.json(
      { error: "Failed to ingest bookmark" },
      { status: 500 }
    );
  }

  return NextResponse.json({
    text: "Successfully embedded bookmark",
  });
}
