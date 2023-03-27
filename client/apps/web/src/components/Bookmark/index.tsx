import { ReactNode, useEffect } from "react";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useSearchParams } from "react-router-dom";
import { animated, useSpring } from "react-spring";
import Chat from "components/Bookmark/Chat";
import axios from "utils/axios";
import { isValidHttpUrl } from "utils/isValidHttpUrl";
import { Circle } from "ui";

export type Bookmark = { url: string; user_id: string; ingested: boolean };
export type User = { username: string; avatar: string; email: string };

const AnimatedCircle = animated(Circle);

const ingestApi = import.meta.env.DEV
  ? `https://0.0.0.0:3000/api/ingest`
  : `/ingest`;

export default function Bookmark() {
  const [searchParams, _setSearchParams] = useSearchParams();
  const bookmarkUrl = searchParams.get("bookmark_url") as string;

  const { data: dataBookmark } = useQuery({
    queryKey: ["bookmark", bookmarkUrl],
    queryFn: async ({ signal }) => {
      let result = await axios.get<Bookmark>("/bookmark", {
        params: { bookmark_url: bookmarkUrl },
        signal,
      });
      return result;
    },
  });

  const { data: dataUser } = useQuery({
    queryKey: ["user"],
    queryFn: async ({ signal }) => {
      let result = await axios.get<User>("/user", {
        signal,
      });
      return result;
    },
  });

  const queryClient = useQueryClient();
  const { mutate: ingestBookmark, status: statusIngestion } = useMutation({
    mutationFn: async () => {
      let result = await axios.post(ingestApi, undefined, {
        params: { bookmark_url: bookmarkUrl },
      });
      return result;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["bookmark", bookmarkUrl] });
    },
  });

  useEffect(() => {
    const isValid = isValidHttpUrl(bookmarkUrl);
    if (isValid && dataBookmark?.data.ingested === false) {
      ingestBookmark();
    }
  }, [dataBookmark]);

  const [circleAnimateProps] = useSpring(
    () => ({
      from: {
        strokeDasharray: 80,
        strokeDashoffset: 80,
      },
      to: { strokeDashoffset: 0 },
      loop: true,
      config: { duration: 1500 },
    }),
    []
  );

  const isValid = isValidHttpUrl(bookmarkUrl);

  return dataUser ? (
    dataBookmark?.data.ingested ? (
      <Chat user={dataUser.data} />
    ) : isValid ? (
      statusIngestion === "loading" ? (
        <Centered>
          <AnimatedCircle
            style={circleAnimateProps}
            className="w-20 stroke-yellow-300 text-center"
          />
          <p className="text-yellow-300">ingesting Mookbark...</p>
        </Centered>
      ) : statusIngestion === "error" ? (
        <Centered>
          <p className="font-bold text-red-300">error ingesting Mookbark!</p>
        </Centered>
      ) : null
    ) : (
      <Centered>
        <p className="font-bold text-red-300">
          Mookbark url doesn't seem to be valid!
        </p>
      </Centered>
    )
  ) : null;
}

function Centered({ children }: { children: ReactNode }) {
  return (
    <div className="flex h-full flex-col items-center justify-center">
      {children}
    </div>
  );
}
