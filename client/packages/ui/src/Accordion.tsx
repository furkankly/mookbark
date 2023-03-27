"use client";

import React from "react";
import * as Accordion from "@radix-ui/react-accordion";
import { ChevronDownIcon } from "@radix-ui/react-icons";

import { cn } from "./utils";

export const AccordionItem = React.forwardRef(
  (
    {
      trigger,
      rcontent,
      value,
      className,
      ...props
    }: {
      trigger: React.ReactNode;
      rcontent: React.ReactNode;
      value: string;
    } & Accordion.AccordionItemProps,
    forwardedRef: React.ForwardedRef<HTMLDivElement>
  ) => (
    <Accordion.Item
      className={className}
      {...props}
      ref={forwardedRef}
      value={value}
    >
      <AccordionTrigger>{trigger}</AccordionTrigger>
      <AccordionContent>{rcontent}</AccordionContent>
    </Accordion.Item>
  )
);
const AccordionTrigger = React.forwardRef(
  (
    {
      children,
      className,
      ...props
    }: {
      children: React.ReactNode;
    } & Accordion.AccordionTriggerProps,
    forwardedRef: React.ForwardedRef<HTMLButtonElement>
  ) => (
    <Accordion.Header>
      <Accordion.Trigger
        className={cn(
          "group flex w-full items-center justify-between text-lg",
          className
        )}
        {...props}
        ref={forwardedRef}
      >
        {children}
        <ChevronDownIcon
          aria-hidden
          className="transition-transform group-data-[state=closed]:rotate-0 group-data-[state=open]:rotate-180"
        />
      </Accordion.Trigger>
    </Accordion.Header>
  )
);

const AccordionContent = React.forwardRef(
  (
    {
      children,
      className,
      ...props
    }: {
      children: React.ReactNode;
    } & Accordion.AccordionContentProps,
    forwardedRef: React.ForwardedRef<HTMLDivElement>
  ) => {
    return (
      <Accordion.Content
        className={cn(
          "data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down overflow-hidden",
          className
        )}
        {...props}
        ref={forwardedRef}
      >
        {children}
      </Accordion.Content>
    );
  }
);
