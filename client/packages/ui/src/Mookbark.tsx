"use client";

import { CSSProperties, forwardRef } from "react";
import { animated } from "@react-spring/web";

import { cn } from "./utils";

export const Mookbark = forwardRef(
  (
    {
      className,
      faceCn,
      faceStyle,
      browsCn,
      browsStyle,
      eyesCn,
      mouthCn,
      mouthStyle,
      ...props
    }: React.SVGAttributes<SVGSVGElement> & {
      faceStyle?: CSSProperties;
      faceCn?: string;
      browsStyle?: CSSProperties;
      browsCn?: string;
      eyesCn?: string;
      mouthStyle?: CSSProperties;
      mouthCn?: string;
    },
    ref: React.ForwardedRef<SVGSVGElement>
  ) => (
    <svg
      ref={ref}
      viewBox="0 0 36 36"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={cn("w-9", className)}
      {...props}
    >
      <animated.path
        d="M16.0214 6.19865C5.77552 1.29253 -1.67609 -2.14943 3.91259 25.6519L15.5326 24.3676L22.4327 34C31.0998 15.6861 46.2775 -7.65619 16.0214 6.19865Z"
        className={cn("fill-yellow-300 stroke-yellow-900", faceCn)}
        style={faceStyle}
      />
      <animated.path
        d="M7.14474 14.1239C11.1144 5.15578 15.411 10.2227 16.4019 13.8936"
        className={cn("stroke-yellow-900", browsCn)}
        style={browsStyle}
      />
      <animated.path
        d="M18.8606 15.5886C22.9822 6.67824 27.1922 11.8059 28.1205 15.4906"
        className={cn("stroke-yellow-900", browsCn)}
        style={browsStyle}
      />
      <path
        d="M15 14.8647C15 15.904 14.0031 16.7464 12.7733 16.7464C11.5435 16.7464 10.5466 15.904 10.5466 14.8647C10.5466 13.8254 9.31679 10 10.5466 10C11.7764 10 15 13.8254 15 14.8647Z"
        className={cn("fill-yellow-600", eyesCn)}
      />
      <path
        d="M25 16C25 16.7055 23.4994 17.5 22.5 17.5C21.5006 17.5 21 16.4281 21 15.7226C21 15.0172 24.7321 12 25.7315 12C26.7308 12 25 15.2945 25 16Z"
        className={cn("fill-yellow-600", eyesCn)}
      />
      <animated.path
        d="M11.3506 17.4883C13.2135 18.3424 20.665 20.0508 22.5279 19.1966C24.3908 18.3424 22.5279 20.905 22.5279 20.905C22.5279 20.905 23.4594 20.905 13.2135 20.905C2.96755 20.905 9.48769 16.6341 11.3506 17.4883Z"
        className={cn("fill-yellow-300 stroke-yellow-900", mouthCn)}
        style={mouthStyle}
      />
    </svg>
  )
);
