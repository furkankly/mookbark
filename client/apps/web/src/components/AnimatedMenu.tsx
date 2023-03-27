import {
  animated,
  config,
  useChain,
  useSpring,
  useSpringRef,
  useTransition,
} from "@react-spring/web";
import { useMutation } from "@tanstack/react-query";
import { useState } from "react";
import { HashLink } from "react-router-hash-link";
import { Mookbark } from "ui";
import axios from "utils/axios";

const items = [
  // { to: "/login#faq", name: "faq" },
  { to: "", name: "logout" },
];

const AnimatedMookbark = animated(Mookbark);
const AnimatedHashLink = animated(HashLink);

const AnimatedMenu = () => {
  const [open, setOpen] = useState(false);

  const springApi = useSpringRef();
  const [props, api] = useSpring(
    () => ({
      ref: springApi,
      config: config.stiff,
      from: {
        x: "-50%",
        strokeDashoffset: 1000,
        strokeDasharray: 1000,
        width: 36,
      },
      to: {
        x: open ? "-150%" : "-50%",
        strokeDashoffset: open ? 0 : 1000,
        strokeDasharray: 1000,
        width: open ? 64 : 36,
      },
    }),
    [open]
  );

  const transApi = useSpringRef();
  const transition = useTransition(open ? items : [], {
    ref: transApi,
    trail: 600 / items.length,
    from: { opacity: 0, x: 0 },
    enter: (_item, index) => ({ opacity: 1, x: 20 * (index + 1) }),
    leave: { opacity: 0, x: 0 },
  });

  useChain(open ? [springApi, transApi] : [transApi, springApi], [0.3, 0.4]);

  const { mutateAsync: logout } = useMutation({
    mutationFn: async () => {
      let result = await axios.post<unknown>("/logout", undefined);
      return result;
    },
  });
  const handleClickLogout = async () => {
    await logout();
    location.href = import.meta.env.VITE_BASE_URL as string;
  };

  return (
    <>
      <animated.button
        style={{ x: props.x, y: "-50%" }}
        className={`group absolute left-1/2 top-1/2 z-10`}
        onClick={() => {
          api.start();
          setOpen((open) => !open);
        }}
      >
        <AnimatedMookbark
          style={{ width: props.width }}
          faceCn="group-hover:fill-yellow-900 group-hover:stroke-yellow-600 group"
          mouthStyle={{
            strokeDasharray: props.strokeDasharray,
            strokeDashoffset: props.strokeDashoffset,
          }}
          browsStyle={{
            strokeDasharray: props.strokeDasharray,
            strokeDashoffset: props.strokeDashoffset,
          }}
          mouthCn="group-hover:fill-yellow-300"
          browsCn="group-hover:stroke-yellow-300"
        />
      </animated.button>
      <div className="flex h-full flex-row items-center justify-center">
        {transition((style, item) =>
          item.name === "faq" ? (
            <AnimatedHashLink
              style={style}
              className="z-10 text-yellow-900"
              to={item.to}
              smooth
            >
              {item.name}
            </AnimatedHashLink>
          ) : (
            <animated.button
              style={style}
              className="z-10 text-yellow-900"
              {...(item.name === "logout" && {
                onClick: () => handleClickLogout(),
              })}
            >
              {item.name}
            </animated.button>
          )
        )}
      </div>
    </>
  );
};

export default AnimatedMenu;
