import { useLayoutEffect, useState } from "react";

const useWindowDimensions = () => {
  const [dimensions, setDimensions] = useState([
    window.innerWidth,
    window.innerHeight,
  ]);

  useLayoutEffect(() => {
    const handleDimensions = () => {
      setDimensions([window.innerWidth, window.innerHeight]);
    };
    window.addEventListener("resize", handleDimensions);
    return () => window.removeEventListener("resize", handleDimensions);
  }, []);

  return dimensions;
};

export default useWindowDimensions;
