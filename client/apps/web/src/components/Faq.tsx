import * as Accordion from "@radix-ui/react-accordion";
import * as Separator from "@radix-ui/react-separator";

import { AccordionItem } from "ui";

const items = [
  {
    question: "Why should I use this?",
    answer: (
      <p>
        Given a problem, you often end up with a bunch of websites while
        searching answers through internet. Each page has a piece of information
        you might need to collect to come up with a solution. Often times, we
        believe that a page link or link groups create a new scope in your
        thinking process towards your goal and Mookbark exists to assist you on
        that journey.{" "}
        <span className="text-yellow-300">
          Mookbark is designed to be more than the typical 'bookmarks' in your
          everyday browser and provides you with{" "}
          <strong>enhanced note-taking capabilities</strong>,{" "}
          <strong>AI assisted suggestions</strong> and other features in the
          baking!
        </span>
      </p>
    ),
  },
];

const Faq = () => (
  <Accordion.Root className="z-10 mx-auto" type="multiple">
    {items.map((item, index) => (
      <div key={index} className="w-96">
        <AccordionItem
          className="rounded bg-yellow-900 p-3"
          key={index}
          trigger={item.question}
          rcontent={<div className="py-4">{item.answer}</div>}
          value={item.question}
        />
        {index !== items.length - 1 && (
          <Separator.Root className="h-0.5 bg-fuchsia-100" />
        )}
      </div>
    ))}
  </Accordion.Root>
);

export default Faq;
