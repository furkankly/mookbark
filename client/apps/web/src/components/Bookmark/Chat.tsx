import { useRef } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import useMeasure from "react-use-measure";
import ReactMarkdown from "react-markdown";
import { useChat } from "ai/react";
import TextareaAutosize from "react-textarea-autosize";
import { ArrowUpIcon, StopIcon } from "@radix-ui/react-icons";
import ScrollToBottom from "react-scroll-to-bottom";
import { Mookbark, Zigzag, twScreens } from "ui";
import { User } from ".";
import ScrollToBottomButton from "components/Bookmark/ScrollToBottomButton";
import { isValidHttpUrl } from "utils/isValidHttpUrl";

const chatApi = import.meta.env.DEV
  ? `https://0.0.0.0:3000/api/chat`
  : `${import.meta.env.VITE_SERVER_URL}/chat`;

export default function Chat({ user }: { user: User }) {
  const navigate = useNavigate();
  const formRef = useRef<HTMLFormElement>(null);
  const [searchParams, _setSearchParams] = useSearchParams();
  const bookmarkUrl = searchParams.get("bookmark_url");

  const { messages, input, handleInputChange, handleSubmit, isLoading, stop } =
    useChat({
      api: `${chatApi}?bookmark_url=${bookmarkUrl}`,
      credentials: import.meta.env.DEV ? "include" : "same-origin",
      onError: (error) => {
        if (error.message.includes("auth")) {
          navigate("/login", { replace: true });
        }
      },
    });

  const [ref, { height }] = useMeasure();
  const amount = Math.floor(height / 192);

  return (
    <div
      ref={ref}
      className="relative flex min-h-0 w-full flex-1 flex-col bg-fuchsia-900/20 p-2 px-4 md:mx-auto md:w-1/2 md:px-20"
    >
      {
        // (hacky) thanks to re-renders ^^
        window.innerWidth >= twScreens.md && (
          <div className="absolute -right-4 top-0 z-10 flex h-full flex-col items-center justify-center">
            {Array.from(Array(amount)).map((_el, index) => (
              <Zigzag className="stroke-fuchsia-100/50 stroke-2" key={index} />
            ))}
          </div>
        )
      }
      <div className="min-h-0 flex-1 pb-9">
        <ScrollToBottom
          className="h-full basis-0 overflow-scroll"
          followButtonClassName="hidden"
        >
          <div className="flex flex-col gap-6 md:px-20">
            {messages.map((m, index) => (
              <div className="break-words border-yellow-300" key={m.id}>
                {m.role === "assistant" ? (
                  <div className="mb-1 flex items-center gap-2">
                    <Mookbark className="w-8 hover:animate-ping" />
                    <p className="text-yellow-300">Mookbark</p>
                  </div>
                ) : m.role === "user" ? (
                  <div
                    className={`mb-1 flex items-center gap-2 ${
                      isLoading && index === messages.length - 1
                        ? "animate-pulse"
                        : ""
                    }`}
                  >
                    {isValidHttpUrl(user.avatar) && (
                      <img
                        className="w-8 rounded-full"
                        src={user.avatar}
                        referrerPolicy="no-referrer"
                      />
                    )}
                    <p
                      className={`bg-yellow-900 text-yellow-300 ${
                        isLoading && index === messages.length - 1
                          ? "animate-pulse"
                          : ""
                      }`}
                    >
                      {user.username}
                    </p>
                  </div>
                ) : (
                  m.role
                )}
                <ReactMarkdown className="prose prose-invert">
                  {m.content}
                </ReactMarkdown>
              </div>
            ))}
          </div>
          <ScrollToBottomButton />
        </ScrollToBottom>
      </div>

      <form className="w-full" ref={formRef} onSubmit={handleSubmit}>
        <div className="flex items-center gap-2">
          <TextareaAutosize
            className="flex-1 resize-none rounded border-yellow-300 p-3 text-black focus:outline-none "
            autoFocus
            minRows={1}
            maxRows={10}
            value={input}
            placeholder="Talk to your mookbark..."
            onChange={handleInputChange}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                if (!isLoading) {
                  e.preventDefault();
                  formRef.current?.requestSubmit();
                }
              }
            }}
          />
          <button
            className="group rounded bg-fuchsia-300/50 p-2 hover:bg-fuchsia-900/50"
            onClick={(event) => {
              event.preventDefault();
              if (isLoading) {
                stop();
              } else {
                formRef.current?.requestSubmit();
              }
            }}
          >
            {isLoading ? (
              <StopIcon className="group-hover:text-yellow-300" />
            ) : (
              <ArrowUpIcon className="group-hover:text-yellow-300" />
            )}
          </button>
        </div>
      </form>
    </div>
  );
}
