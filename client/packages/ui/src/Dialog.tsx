"use client";

import React from "react";
import * as Dialog from "@radix-ui/react-dialog";

import { cn } from "./utils";

// TODO: Make use of this
export const DialogContent = React.forwardRef(
  (
    {
      trigger,
      content,
      value,
      className,
      ...props
    }: {
      trigger: React.ReactNode;
      content: React.ReactNode;
      value: string;
    } & Dialog.DialogContentProps,
    forwardedRef: React.ForwardedRef<HTMLDivElement>
  ) => (
    <Dialog.Content
      className={cn("", className)}
      {...props}
      ref={forwardedRef}
    />
  )
);
