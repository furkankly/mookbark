import { Fragment } from "react";
import { useQuery } from "@tanstack/react-query";
import useMeasure from "react-use-measure";
import { HorizontalZigzag, HorizontalZigzag2, Zigzag } from "ui";
import axios from "utils/axios";
import AddBookmark from "../Bookmarks/AddBookmark";
import AddContainer from "../Bookmarks/AddContainer";
import { MemoizedFullTree as Tree } from "./Tree";

export type Tree = { root: string; leaves: Tree[] };

const Loading = () => {
  const zigzags = Array(10)
    .fill(null)
    .map((_, index) => {
      const width = Math.floor(80 * Math.random() + 60);
      return (
        <Fragment key={index}>
          <HorizontalZigzag
            style={{ width }}
            className="animate-pulse stroke-fuchsia-100/50 grayscale"
          />
          <HorizontalZigzag2
            style={{ width }}
            className="animate-pulse stroke-fuchsia-100/50"
          />
        </Fragment>
      );
    });
  return <div className="flex flex-col gap-2 p-2">{zigzags}</div>;
};

export default function Bookmarks() {
  const { data, status } = useQuery({
    queryKey: ["bookmarks"],
    queryFn: async ({ signal }) => {
      let result = await axios.get<Tree>("/bookmarks", {
        signal,
      });
      return result;
    },
  });

  const [ref, { height }] = useMeasure();
  const amount = Math.floor(height / 192);

  return (
    <div
      ref={ref}
      className="relative my-20 mr-4 flex-1 rounded-xl bg-yellow-300/20 p-10 sm:mx-16 md:mx-32 lg:mx-48 xl:mx-64 2xl:mx-96"
    >
      <div className="absolute -right-4 top-0 z-10 flex h-full flex-col items-center justify-center">
        {Array.from(Array(amount)).map((_el, index) => (
          <Zigzag className="stroke-fuchsia-100/50 stroke-2" key={index} />
        ))}
      </div>
      <div className="absolute left-2 top-0 -translate-y-1/2 rounded bg-fuchsia-100/50 p-4 text-2xl">
        Your Mookbarks
      </div>
      <div className="absolute right-2 top-0 flex -translate-y-1/2 gap-2 rounded bg-fuchsia-100/50 p-2 ">
        <AddBookmark />
        <AddContainer />
      </div>
      {status === "loading" ? (
        <Loading />
      ) : status === "error" ? (
        <div>error fetching mookbarks...</div>
      ) : data.data.leaves.length ? (
        <div className="overflow-hidden">
          <Tree data={data.data} />
        </div>
      ) : (
        <div className="flex h-full w-full flex-col items-center justify-center gap-1">
          <p className="text-lg">
            Very <strong className="text-yellow-300">h0ll0w..</strong>
          </p>
          <img src="Empty.png" />
        </div>
      )}
    </div>
  );
}
