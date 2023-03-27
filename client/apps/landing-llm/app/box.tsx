import Image from "next/image";
import { Mookbark } from "ui";

export default function Box() {
  return (
    <div className="p-44">
      <div className="perspective-[700px] perspective-origin-[50%_100px] w-[300px]">
        <div className="transform-style-3d animate-spin-y-40 relative w-[300px]">
          <div
            style={{
              transform: "rotateX(90deg) translateY(150px)",
              boxShadow: "0px 0px 100px 50px #fde047",
            }}
            className="absolute top-0 h-[300px] w-[300px] origin-[bottom_center]"
          ></div>
        </div>
      </div>
      <div className="perspective-[700px] perspective-origin-[50%_100px] h-[300px] w-[300px]">
        <div className="transform-style-3d animate-spin-y-40 group relative w-[300px] transform-gpu">
          <div
            style={{ transform: "translateZ(150px)" }}
            className="group-has-[img:hover]:outline-none absolute flex h-[300px] w-[300px] flex-col opacity-75 shadow-yellow-900 outline outline-yellow-300 transition-opacity duration-200 hover:opacity-100"
          >
            <p className="bg-yellow-900 text-center font-bold text-yellow-300">
              Bookmarks on the web
            </p>
            <div className="relative flex-1">
              <Image
                className="duration-200 hover:scale-150"
                src="/web.png"
                alt="web"
                // width="600"
                // height="450"
                fill
                priority
              />
            </div>
          </div>
          <div
            style={{ transform: "translateZ(-150px) rotateY(180deg)" }}
            className="group-has-[img:hover]:outline-none absolute flex h-[300px] w-[300px] flex-col opacity-75 shadow-yellow-900 outline outline-yellow-300 transition-opacity duration-200 hover:opacity-100"
          >
            <p className="bg-yellow-900 text-center font-bold text-yellow-300">
              Bookmarks in your terminal(TUI)
            </p>
            <div className="relative flex-1">
              <Image
                className="duration-200 hover:scale-150"
                src="/tui.png"
                alt="web"
                // width="670"
                // height="670"
                fill
                priority
              />
            </div>
          </div>
          <div
            style={{ transform: "rotateX(-90deg) translateY(-150px)" }}
            className="absolute flex h-[300px] w-[300px] origin-[top_center] flex-col opacity-75 shadow-yellow-900 transition-opacity duration-200 hover:opacity-100"
          >
            <Mookbark className="h-full w-full bg-yellow-900" />
          </div>
          <div
            style={{ transform: "rotateX(90deg) translateY(150px)" }}
            className="absolute flex h-[300px] w-[300px] origin-[bottom_center] flex-col opacity-75 shadow-yellow-900 transition-opacity duration-200 hover:opacity-100"
          >
            <Mookbark className="h-full w-full bg-yellow-900" />
          </div>
          <div
            style={{ transform: "rotateY(270deg) translateX(-150px)" }}
            className="group-has-[img:hover]:outline-none absolute flex h-[300px] w-[300px] origin-[center_left] flex-col opacity-75 shadow-yellow-900 outline outline-yellow-300 transition-opacity duration-200 hover:opacity-100"
          >
            <p className="bg-yellow-900 text-center font-bold text-yellow-300">
              Talk to your bookmarks
            </p>
            <div className="relative flex-1">
              <Image
                className="h-full w-full duration-200 hover:scale-150"
                src="/chat.png"
                alt="web"
                // width="460"
                // height="460"
                fill
                priority
              />
            </div>
          </div>
          <div
            style={{ transform: "rotateY(-270deg) translateX(150px)" }}
            className="group-has-[img:hover]:outline-none absolute flex h-[300px] w-[300px] origin-[top_right] flex-col opacity-75 shadow-yellow-900 outline outline-yellow-300 transition-opacity duration-200 hover:opacity-100"
          >
            <p className="bg-yellow-900 text-center font-bold text-yellow-300">
              Bookmarks in your terminal(CLI)
            </p>
            <div className="relative flex-1">
              <Image
                className="h-full w-full duration-200 hover:scale-150"
                src="/cli.png"
                alt="web"
                // width="670"
                // height="770"
                fill
                priority
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
