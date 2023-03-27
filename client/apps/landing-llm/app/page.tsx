import { Mookbark } from "ui";
import Box from "./box";
import Zigzags from "./zigzags";
import LoginButton from "./login-button";

export default function Home() {
  return (
    <main className="relative h-full overflow-y-auto overflow-x-hidden">
      <div className="absolute top-0 flex w-full justify-end border-b-2 border-b-fuchsia-100 bg-yellow-600 p-2">
        <LoginButton />
      </div>
      <div className="mx-auto flex flex-col items-center justify-between p-20 xl:min-h-screen xl:max-w-screen-2xl xl:flex-row">
        <div className="flex flex-col gap-1">
          <h1 className="relative text-4xl font-bold text-yellow-300 md:text-6xl">
            <Mookbark className="-mr-3 inline w-20 -rotate-12 hover:animate-ping md:-mr-6 md:w-40" />
            ookbark
          </h1>
          <p className="text-xl font-semibold text-white md:text-3xl">
            Manage and Talk to Your Bookmarks for{" "}
            <span className="font-bold text-yellow-300">FREE</span>
          </p>
        </div>
        <Box />
      </div>
      <div className="relative w-full">
        <Zigzags />
      </div>
      <div className="mx-auto flex flex-col items-stretch justify-between gap-10 rounded bg-yellow-600 p-14 text-white xl:max-w-screen-2xl xl:flex-row">
        <div className="flex w-full flex-col gap-10 rounded border-2 border-yellow-300 p-6 xl:flex-1">
          <h4 className="text-xl font-semibold">
            Organize, access and talk to your bookmarks with the dedicated web
            application.
          </h4>
          <div className="flex flex-1 items-center justify-center">
            <LoginButton />
          </div>
        </div>
        <div className="flex w-full flex-col gap-10 rounded border-2 border-yellow-300 p-6 xl:flex-1">
          <h4 className="text-xl font-semibold">
            Manage and access your bookmarks without leaving your terminal with
            the Rust-powered TUI and CLI.
          </h4>
          <div className="flex justify-center">
            <p className="mt-1 text-left font-semibold text-yellow-900">
              Apple Silicon only (for now) <br />
              <code className="text-base">
                brew tap furkankly/tap
                <br /> brew install mookbark
              </code>
            </p>
          </div>
        </div>
      </div>
      <div className="absolute bottom-0 left-0 right-0 top-0 z-[-1] hidden h-full w-full xl:flex">
        <div
          style={{
            opacity: 0.4,
            background:
              "radial-gradient(54.14% 54.14% at 50% 50%, rgb(253 224 71 / 0.8) 0%, rgb(113 63 18 / 0.2) 100%)",
          }}
          className="h-full w-1/2 blur-xl"
        />
      </div>
    </main>
  );
}
