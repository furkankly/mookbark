import { load } from "cheerio";
import natural from "natural";
const { WordTokenizer } = natural;

import { Page } from "langchain/document_loaders/web/puppeteer";

function removeHtmlAndJs(htmlContent: string): string {
  const $ = load(htmlContent);

  $("script").remove();
  $("style").remove();

  // Remove all HTML tags
  return $.text();
}

function removeUnnecessaryLines(content: string): string {
  const lines = content.split("\n");
  const strippedLines = lines.map((line) => line.trim());
  const nonEmptyLines = strippedLines.filter((line) => line !== "");
  const dedupedLines = [...new Set(nonEmptyLines)];
  const cleanedContent = dedupedLines.join("");

  return cleanedContent;
}

export async function evaluateScrape(page: Page): Promise<string> {
  let results = "";

  try {
    const pageSource = await page.content();
    results = removeUnnecessaryLines(removeHtmlAndJs(pageSource));

    const tokenizer = new WordTokenizer();
    results = tokenizer.tokenize(results)?.join("") ?? "";
  } catch (error) {
    results = `Error: ${error}`;
  }
  return results;
}

export function evaluateStaticScrape(htmlContent: string): string {
  let results = "";

  try {
    results = removeUnnecessaryLines(removeHtmlAndJs(htmlContent));

    const tokenizer = new WordTokenizer();
    results = tokenizer.tokenize(results)?.join("") ?? "";
  } catch (error) {
    results = `Error: ${error}`;
  }
  return results;
}
