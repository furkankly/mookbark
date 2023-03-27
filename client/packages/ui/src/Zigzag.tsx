"use client";

import { forwardRef } from "react";

export const Zigzag = forwardRef(
  (
    { className, ...props }: React.HTMLAttributes<SVGSVGElement>,
    ref: React.ForwardedRef<SVGSVGElement>
  ) => {
    return (
      <svg
        ref={ref}
        width="36"
        height="192"
        viewBox="0 0 36 192"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        {...props}
        {...(className && { className })}
      >
        <path
          style={{ zIndex: 10, position: "relative" }}
          strokeWidth="3px"
          d="M1.50003 -3.05176e-05C1.5 19.5 33.5 7.35567 33.5 11.8823C33.5 16.4089 22.2493 22.6329 7.62333 28.2912C-7.00264 33.9494 33.5 49.2267 33.5 49.2267C33.5 49.2267 24.4994 59.4115 7.62333 67.333C-9.25278 75.2546 33.5 84.3078 33.5 89.4002C33.5 94.4926 7.62333 100.717 7.62333 107.507C7.62333 114.296 33.5 116.56 33.5 116.56C33.5 116.56 5 138.447 5 147.5C5 156.553 33.5 165.221 33.5 172.01C33.5 178.8 1.99455 170 2 192"
        />
      </svg>
    );
  }
);
