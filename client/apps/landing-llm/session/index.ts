// import { Redis } from "@upstash/redis";
import { cookies } from "next/headers";
import { decode } from "msgpack-lite";

function findKeyInNestedArray(array: [], key: string): string | undefined {
  for (let i = 0; i < array.length; i++) {
    const element = array[i];
    if (Array.isArray(element)) {
      const found = findKeyInNestedArray(element, key);
      if (found !== undefined) {
        return found;
      }
    } else if (typeof element === "object" && element !== null) {
      if (key in element) {
        return element[key];
      }
    }
  }
  return undefined; // Key not found
}

// I cant get this to decode properly
// const redis = new Redis({
//   url: process.env.REDIS_URL as string,
//   token: process.env.REDIS_TOKEN as string,
//   responseEncoding: "base64",
//   automaticDeserialization: false,
// });

export async function getSession(): Promise<string> {
  const c = cookies();
  const session = c.get("id");
  if (session) {
    return fetch(`${process.env.REDIS_URL}/get/${session.value}`, {
      headers: {
        Authorization: `Bearer ${process.env.REDIS_TOKEN}`,
        "Upstash-Encoding": "base64",
      },
    })
      .then((res) => {
        if (res.ok) return res.json();
        else {
          return Promise.reject("Session couldn't be fetched");
        }
      })
      .then((base64encoded: { result: string }) => {
        const base64decoded = Buffer.from(base64encoded.result, "base64");
        const data = decode(base64decoded);
        const userId = findKeyInNestedArray(data, "user_id");
        if (userId) return userId;
        else return Promise.reject("Session has an unexpected structure");
      });
    // if (session) {
    //   return redis.get(session.value).then((msgpackResponse: any) => {
    //     if (msgpackResponse) {
    //       const decodedData = decode(msgpackResponse);
    //       return decodedData;
    //     } else {
    //       return Promise.reject("Session is empty");
    //     }
    //   });
  } else {
    return Promise.reject("Session cookie not found");
  }
}
