"use client";

import { forwardRef } from "react";
import { cn } from "./utils";

export const HorizontalZigzag2 = forwardRef(
  (
    { className, ...props }: React.HTMLAttributes<SVGSVGElement>,
    ref: React.ForwardedRef<SVGSVGElement>
  ) => {
    return (
      <svg
        ref={ref}
        width="62"
        viewBox="0 0 62 16"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        className={cn("stroke-white", className)}
        {...props}
      >
        <path d="M1 16C1 16 1 0.621953 6.31814 6.5018C11.6363 12.3816 11.6363 8.08211 11.6363 8.08211C11.6363 8.08211 10.7395 2.19541 11.6363 1.7378C12.533 1.28019 22.2726 6.5018 24.5518 6.5018C26.831 6.5018 43.59 -1.40333 28.3504 1.7378C13.1108 4.87893 43.2222 1.64883 43.2222 3.62831C43.2222 5.60778 55.0741 1.7378 56.5556 1.15397C58.037 0.570133 61 3.62831 61 3.62831" />
      </svg>
    );
  }
);
