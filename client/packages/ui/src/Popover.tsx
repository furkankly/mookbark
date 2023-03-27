"use client";

import { Cross2Icon } from "@radix-ui/react-icons";
import * as RadixPopover from "@radix-ui/react-popover";
import React, { forwardRef, useImperativeHandle, useRef } from "react";
import { cn } from "./utils";

export const Popover = forwardRef(
  (
    {
      open,
      onOpenChange,
      popoverTrigger,
      children,
      popoverContentProps,
      popoverArrowProps,
      popoverCloseProps,
    }: {
      open?: RadixPopover.PopoverProps["open"];
      onOpenChange?: RadixPopover.PopoverProps["onOpenChange"];
      popoverTrigger: React.ReactNode;
      children: React.ReactNode;
      popoverContentProps?: React.ComponentPropsWithoutRef<
        typeof RadixPopover.Content
      >;
      popoverArrowProps?: React.ComponentPropsWithoutRef<
        typeof RadixPopover.Arrow
      >;
      popoverCloseProps?: React.ComponentPropsWithoutRef<
        typeof RadixPopover.Close
      >;
    },
    ref: React.ForwardedRef<HTMLDivElement>
  ) => {
    const popoverContentRef = useRef<HTMLDivElement>(null);
    const popoverArrowRef = useRef<SVGSVGElement>(null);
    const popoverCloseRef = useRef<HTMLButtonElement>(null);

    useImperativeHandle<
      unknown,
      {
        popoverContent: HTMLDivElement | null;
        popoverArrow: SVGSVGElement | null;
        popoverClose: HTMLButtonElement | null;
      }
    >(ref, () => {
      return {
        get popoverContent() {
          return popoverContentRef.current;
        },
        get popoverArrow() {
          return popoverArrowRef.current;
        },
        get popoverClose() {
          return popoverCloseRef.current;
        },
      };
    });

    return (
      <RadixPopover.Root open={open} onOpenChange={onOpenChange}>
        <RadixPopover.Trigger asChild>{popoverTrigger}</RadixPopover.Trigger>
        <RadixPopover.Portal>
          <RadixPopover.Content
            ref={popoverContentRef}
            sideOffset={5}
            {...(popoverContentProps ?? {})}
            className={cn(
              "data-[state=open]:data-[side=top]:animate-slide-down-and-fade data-[state=open]:data-[side=bottom]:animate-slide-up-and-fade data-[state=open]:data-[side=left]:animate-slide-right-and-fade data-[state=open]:data-[side=right]:animate-slide-left-and-fade relative relative z-10 rounded bg-fuchsia-300/50 p-4 shadow-fuchsia-100/50",
              popoverContentProps?.className
            )}
          >
            {children}
            <RadixPopover.Close
              ref={popoverCloseRef}
              aria-label="Close"
              {...(popoverCloseProps ?? {})}
              className={cn(
                "absolute right-1 top-1",
                popoverCloseProps?.className
              )}
            >
              <Cross2Icon />
            </RadixPopover.Close>
            <RadixPopover.Arrow
              ref={popoverArrowRef}
              {...(popoverArrowProps ?? {})}
              className={cn(
                "fill-fuchsia-300/50",
                popoverArrowProps?.className
              )}
            />
          </RadixPopover.Content>
        </RadixPopover.Portal>
      </RadixPopover.Root>
    );
  }
);
