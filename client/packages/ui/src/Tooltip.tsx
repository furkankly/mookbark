"use client";

import React from "react";
import * as RadixTooltip from "@radix-ui/react-tooltip";

import { cn } from "./utils";

export const Tooltip = React.forwardRef(
  (
    {
      trigger,
      rcontent,
      className,
      ...props
    }: {
      trigger: React.ReactNode;
      rcontent: React.ReactNode;
    } & RadixTooltip.TooltipTriggerProps,
    ref: React.ForwardedRef<HTMLButtonElement>
  ) => {
    return (
      <RadixTooltip.Provider>
        <RadixTooltip.Root>
          <RadixTooltip.Trigger
            className={className}
            {...props}
            ref={ref}
            asChild
          >
            {trigger}
          </RadixTooltip.Trigger>
          <RadixTooltip.Portal>
            <TooltipContent>{rcontent}</TooltipContent>
          </RadixTooltip.Portal>
        </RadixTooltip.Root>
      </RadixTooltip.Provider>
    );
  }
);

const TooltipContent = React.forwardRef(
  (
    {
      children,
      className,
      ...props
    }: {
      children: React.ReactNode;
    } & RadixTooltip.TooltipContentProps,
    forwardedRef: React.ForwardedRef<HTMLDivElement>
  ) => {
    return (
      <RadixTooltip.Content
        className={cn(
          "z-10 max-w-xl break-words rounded border border-yellow-300 bg-yellow-900 p-2 text-yellow-300",
          className
        )}
        sideOffset={5}
        {...props}
        ref={forwardedRef}
      >
        {children}
      </RadixTooltip.Content>
    );
  }
);
