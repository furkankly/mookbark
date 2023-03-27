"use client";

import { HorizontalZigzag } from "./HorizontalZigzag";
import React, { forwardRef } from "react";
import { cn } from "./utils";

export const Button = forwardRef(
  (
    { children, className, ...props }: React.HTMLAttributes<HTMLButtonElement>,
    ref: React.ForwardedRef<HTMLButtonElement>
  ) => {
    return (
      <button
        ref={ref}
        className={cn(
          "group relative rounded bg-yellow-900 p-2 text-yellow-300 ring-inset ring-yellow-300 hover:bg-yellow-300 hover:text-yellow-900 focus:ring-1",
          className
        )}
        {...props}
      >
        <HorizontalZigzag className="absolute -bottom-1 left-0 w-16 stroke-yellow-300 stroke-2 group-hover:stroke-yellow-900" />
        {children}
      </button>
    );
  }
);
