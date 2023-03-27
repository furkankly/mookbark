"use client";

import { forwardRef } from "react";

export const GridZigzag = forwardRef(
  (
    { className, ...props }: React.HTMLAttributes<SVGSVGElement>,
    ref: React.ForwardedRef<SVGSVGElement>
  ) => {
    return (
      <svg
        ref={ref}
        width="27"
        viewBox="0 0 27 27"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        {...props}
        {...(className && { className })}
      >
        <path d="M10.1646 26.1673C10.1646 26.1673 19.5569 13.5737 16.6063 6.14142C13.6558 -1.29087 -1.90572 0.360678 1.71538 6.14138C5.33648 11.9221 26.1722 17.9554 26.1722 17.9554" />
      </svg>
    );
  }
);
