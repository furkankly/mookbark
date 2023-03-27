"use client";

import useMeasure from "react-use-measure";
import { HorizontalZigzag2 } from "ui";

export default function Zigzags() {
  const [ref, { width }] = useMeasure();
  const amount = Math.floor(width / 62);

  const zigzags = (
    <div ref={ref} className="absolute top-0 flex w-full justify-center">
      {Array.from(Array(amount)).map((_el, index) => (
        <HorizontalZigzag2 className="stroke-yellow-300 stroke-2" key={index} />
      ))}
    </div>
  );

  return zigzags;
}
