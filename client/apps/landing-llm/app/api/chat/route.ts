import { NextRequest, NextResponse } from "next/server";

import { Message as VercelChatMessage, StreamingTextResponse } from "ai";

import { PineconeStore } from "@langchain/pinecone";
import { HumanMessage, AIMessage, ChatMessage } from "@langchain/core/messages";
import {
  ChatPromptTemplate,
  MessagesPlaceholder,
} from "@langchain/core/prompts";
import { RunnableSequence, RunnablePick } from "@langchain/core/runnables";
import { ChatTogetherAI } from "@langchain/community/chat_models/togetherai";
import { TogetherAIEmbeddings } from "@langchain/community/embeddings/togetherai";
import { Pinecone } from "@pinecone-database/pinecone";
import { createStuffDocumentsChain } from "langchain/chains/combine_documents";
import { createRetrievalChain } from "langchain/chains/retrieval";
import { createHistoryAwareRetriever } from "langchain/chains/history_aware_retriever";
import { HttpResponseOutputParser } from "langchain/output_parsers";

const formatVercelMessages = (message: VercelChatMessage) => {
  if (message.role === "user") {
    return new HumanMessage(message.content);
  } else if (message.role === "assistant") {
    return new AIMessage(message.content);
  } else {
    console.warn(
      `Unknown message type passed: "${message.role}". Falling back to generic message type.`,
    );
    return new ChatMessage({ content: message.content, role: message.role });
  }
};

const ANSWER_SYSTEM_TEMPLATE = `You are a helpful AI assistant. Use the following pieces of context to answer the question at the end.
If you don't know the answer, just say you don't know. DO NOT try to make up an answer.
If the question is not related to the context, politely respond that you are tuned to only answer questions that are related to the context.

<context>
{context}
</context>

Please return your answer in markdown with clear headings and lists.`;

const answerPrompt = ChatPromptTemplate.fromMessages([
  ["system", ANSWER_SYSTEM_TEMPLATE],
  new MessagesPlaceholder("chat_history"),
  ["user", "{input}"],
]);

const historyAwarePrompt = ChatPromptTemplate.fromMessages([
  new MessagesPlaceholder("chat_history"),
  ["user", "{input}"],
  [
    "user",
    "Given the above conversation, generate a concise vector store search query to look up in order to get information relevant to the conversation.",
  ],
]);

const pinecone = new Pinecone({
  apiKey: process.env.PINECONE_API_KEY ?? "",
});

export async function POST(req: NextRequest) {
  try {
    const bookmarkUrl = req.nextUrl.searchParams.get("bookmark_url");

    if (bookmarkUrl == null) {
      return NextResponse.json(
        { error: "No bookmark url found" },
        { status: 400 },
      );
    }

    const body = await req.json();
    const messages = body.messages ?? [];
    if (!messages.length) {
      throw new Error("No messages provided.");
    }
    const formattedPreviousMessages = messages
      .slice(0, -1)
      .map(formatVercelMessages);
    const currentMessageContent = messages[messages.length - 1].content;

    const embeddings = new TogetherAIEmbeddings({
      modelName: "togethercomputer/m2-bert-80M-8k-retrieval",
    });

    const PINECONE_INDEX_NAME = process.env.PINECONE_INDEX_NAME ?? "";
    const index = pinecone.index(PINECONE_INDEX_NAME);
    const vectorstore = await PineconeStore.fromExistingIndex(embeddings, {
      pineconeIndex: index,
      namespace: bookmarkUrl,
    });
    const retriever = vectorstore.asRetriever();

    const chatModel = new ChatTogetherAI({
      modelName: "mistralai/Mixtral-8x7B-Instruct-v0.1",
      temperature: 0,
    });

    const historyAwareRetrieverChain = await createHistoryAwareRetriever({
      llm: chatModel,
      retriever,
      rephrasePrompt: historyAwarePrompt,
    });

    const documentChain = await createStuffDocumentsChain({
      llm: chatModel,
      prompt: answerPrompt,
    });

    const conversationalRetrievalChain = await createRetrievalChain({
      combineDocsChain: documentChain,
      retriever: historyAwareRetrieverChain,
    });

    const outputChain = RunnableSequence.from([
      conversationalRetrievalChain,
      new RunnablePick({ keys: "answer" }),
      new HttpResponseOutputParser({ contentType: "text/plain" }),
    ]);

    const stream = await outputChain.stream({
      chat_history: formattedPreviousMessages,
      input: currentMessageContent,
    });

    return new StreamingTextResponse(stream);
  } catch (e: any) {
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}
