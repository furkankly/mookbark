import { a, animated, useSpring } from "@react-spring/web";
import { memo, ReactNode, useState } from "react";
import { createSearchParams, useNavigate } from "react-router-dom";
import useMeasure from "react-use-measure";
import { ChatBubbleIcon } from "@radix-ui/react-icons";
import { HorizontalZigzag, Tooltip } from "ui";
import { isValidHttpUrl } from "utils/isValidHttpUrl";
import { type Tree } from ".";
import AddBookmark from "./AddBookmark";
import AddContainer from "./AddContainer";
import DeleteEntity from "./DeleteEntity";

const AnimatedHorizontalZigzag = animated(HorizontalZigzag);

const Tree = ({ name, children }: { name: string; children?: ReactNode }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [childrenContainerRef, { height: childrenContainerViewHeight }] =
    useMeasure();

  const childrenContainerProps = useSpring({
    from: { height: 0, opacity: 0, y: 0 },
    to: {
      height: isOpen ? childrenContainerViewHeight : 0,
      opacity: isOpen ? 1 : 0,
      y: isOpen ? 0 : 200,
    },
  });

  const [horizontalZigZagAnimateProps, api] = useSpring(
    () => ({
      from: {
        stroke: "white",
        strokeDasharray: 1000,
        strokeDashoffset: 1000,
      },
    }),
    [childrenContainerProps]
  );

  const navigate = useNavigate();

  return (
    <a.div
      className="group relative m-2 cursor-pointer text-xl"
      onClick={(event) => {
        event.stopPropagation();
        setIsOpen((prevIsOpen) => !prevIsOpen);
      }}
      onMouseEnter={() => {
        api.start({ to: { stroke: "#fde047", strokeDashoffset: 0 } });
      }}
      onMouseLeave={() => {
        api.start({ to: { stroke: "white", strokeDashoffset: 1000 } });
      }}
    >
      <div className="relative flex items-center gap-1">
        {!isValidHttpUrl(name) ? (
          <>
            <AddBookmark containerName={name} />
            <AddContainer parentContainerName={name} />
            <DeleteEntity entityType="container" name={name} />
            <p
              className={`min-w-0 break-words text-base md:text-lg ${
                children && "text-yellow-300"
              }`}
            >
              {name}
            </p>
          </>
        ) : (
          <>
            <button
              className="group/talkToMookbark rounded bg-transparent p-1 hover:bg-yellow-900"
              onClick={(e: React.MouseEvent) => {
                e.stopPropagation();
                navigate({
                  search: createSearchParams({ bookmark_url: name }).toString(),
                });
              }}
            >
              <ChatBubbleIcon className="group-hover/talkToMookbark:text-yellow-300" />
            </button>
            <DeleteEntity entityType="bookmark" name={name} />
            <AnimatedHorizontalZigzag
              style={horizontalZigZagAnimateProps}
              className="absolute -bottom-1 left-14 w-16 stroke-yellow-300 stroke-2"
            />
            <Tooltip
              onClick={(event) => {
                event.stopPropagation();
              }}
              trigger={
                <a
                  href={`${name}`}
                  target="_blank"
                  className="overflow-hidden text-ellipsis whitespace-nowrap text-sm md:text-base"
                >
                  {name}
                </a>
              }
              rcontent={name}
            />
          </>
        )}
      </div>
      {children && (
        <a.div
          style={{
            height: childrenContainerProps.height,
            opacity: childrenContainerProps.opacity,
          }}
          className="m-2"
        >
          <a.div
            ref={childrenContainerRef}
            style={{ y: childrenContainerProps.y }}
          >
            {children}
          </a.div>
        </a.div>
      )}
    </a.div>
  );
};

const FullTree = ({ data }: { data: Tree }) => {
  return data.root === "root" ? (
    <>
      {data.leaves.map((tree) => (
        <FullTree key={tree.root} data={tree} />
      ))}
    </>
  ) : (
    <Tree name={data.root}>
      {data.leaves.length
        ? data.leaves.map((tree) => <FullTree key={tree.root} data={tree} />)
        : null}
    </Tree>
  );
};

export const MemoizedFullTree = memo(FullTree);
