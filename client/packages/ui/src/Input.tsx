"use client";

import { HorizontalZigzag } from "./HorizontalZigzag";
import { forwardRef } from "react";
import { cn } from "./utils";

export const Input = forwardRef(
  (
    { className, type, ...props }: React.InputHTMLAttributes<HTMLInputElement>,
    ref: React.ForwardedRef<HTMLInputElement>
  ) => {
    return (
      <div className="group relative">
        <input
          type={type}
          className={cn(
            "flex h-10 w-full rounded-md border px-3 py-2 text-sm text-fuchsia-900 ring-fuchsia-900 file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-fuchsia-900/50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
            className
          )}
          ref={ref}
          {...props}
        />
        <HorizontalZigzag className="absolute -bottom-1 left-0 w-16 stroke-fuchsia-900 stroke-2 group-hover:stroke-fuchsia-300" />
      </div>
    );
  }
);
