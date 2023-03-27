"use client";

import { forwardRef } from "react";
import { cn } from "./utils";

export const HorizontalZigzag = forwardRef(
  (
    { className, ...props }: React.HTMLAttributes<SVGSVGElement>,
    ref: React.ForwardedRef<SVGSVGElement>
  ) => {
    return (
      <svg
        ref={ref}
        width="62"
        viewBox="0 0 62 8"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        className={cn("stroke-white", className)}
        {...props}
      >
        <path d="M1 7.04002C1 7.04002 5.7619 -2.33496 7.66667 2.35253C9.57143 7.04003 15.7619 7.04002 15.7619 7.04002C15.7619 7.04002 25.2857 5.70074 24.8095 2.35253C24.3333 -0.995671 36.7143 3.69181 36.7143 3.69181C36.7143 3.69181 41.4762 7.04002 50.3671 7.04002C59.258 7.04002 24.8095 1.01322 61 2.35253" />
      </svg>
    );
  }
);
