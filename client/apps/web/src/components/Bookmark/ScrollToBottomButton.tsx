import { useScrollToBottom, useSticky } from "react-scroll-to-bottom";
import { ArrowDownIcon } from "@radix-ui/react-icons";

export default function ScrollToBottomButton() {
  const scrollToBottom = useScrollToBottom();
  const [sticky] = useSticky();

  return !sticky ? (
    <button
      className="group absolute bottom-4 right-1/2 animate-bounce rounded-full bg-yellow-300/50 p-1 p-2 hover:bg-yellow-300/80"
      onClick={() => {
        scrollToBottom();
      }}
    >
      <ArrowDownIcon className="group-hover/text-yellow-300" />
    </button>
  ) : null;
}
