// import Faq from "components/Faq";
import useOAuthLogin from "hooks/useOAuthLogin";
import { useNavigate } from "react-router-dom";
import useMeasure from "react-use-measure";
import {
  Button,
  GridZigzag,
  HorizontalZigzag2,
  Mookbark,
  twScreens,
  Zigzag,
} from "ui";

const MookbarksBackground = () => {
  return (
    <>
      <Mookbark className="absolute left-72 top-32 w-8 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
      <Mookbark className="absolute left-2/4 top-10 w-5 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
      <Mookbark className="absolute left-80 top-48 w-7 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
      <Mookbark className="absolute left-1/3 top-64 w-3 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
      <Mookbark className="absolute left-2/3 top-80 w-8 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
      <Mookbark className="absolute bottom-80 left-72 w-8 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
      <Mookbark className="absolute bottom-10 right-1/4 w-10 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
      <Mookbark className="absolute bottom-24 right-2/4 w-6 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
      <Mookbark className="absolute bottom-40 right-3/4 w-6 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
      <Mookbark className="absolute bottom-56 right-1/3 w-2 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
      <Mookbark className="absolute bottom-72 right-72 w-8 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
      <Mookbark className="absolute bottom-48 right-72 w-8 rotate-12 grayscale hover:animate-ping hover:grayscale-0" />
    </>
  );
};

const Login = () => {
  const state = self.crypto.randomUUID();

  if (!sessionStorage.getItem("state")) sessionStorage.setItem("state", state);

  const navigate = useNavigate();
  useOAuthLogin({
    onSuccess: () => {
      sessionStorage.removeItem("state");
      navigate("/", { replace: true });
    },
  });

  const handleLogin = (oauthProvider: "google" | "discord") => {
    let authUrl = "";
    if (oauthProvider === "google") {
      authUrl = `${
        import.meta.env.VITE_OAUTH_GOOGLE_AUTH_URL
      }&state=${sessionStorage.getItem("state")}`;
    } else if (oauthProvider === "discord") {
      authUrl = `${
        import.meta.env.VITE_OAUTH_DISCORD_AUTH_URL
      }&state=${sessionStorage.getItem("state")}`;
    }
    window.open(authUrl, "_self");
  };

  const [ref, { width, height }] = useMeasure();
  const amount = Math.floor(height / 192);

  const leftEdges = (
    <div className="absolute left-[calc(25%-18px)] top-0 flex h-full flex-col items-center justify-center">
      {Array.from(Array(amount)).map((_el, index) => (
        <Zigzag className="stroke-yellow-300 stroke-2" key={index} />
      ))}
    </div>
  );
  const rightEdges = (
    <div className="absolute right-[calc(25%-18px)] top-0 flex h-full flex-col items-center justify-center">
      {Array.from(Array(amount)).map((_el, index) => (
        <Zigzag className="stroke-yellow-300 stroke-2" key={index} />
      ))}
    </div>
  );

  return (
    <div ref={ref} className="relative bg-yellow-600">
      {<MookbarksBackground />}
      {width >= twScreens.md && leftEdges}
      <div className="mx-auto flex h-screen w-full flex-col items-center justify-center border-x-2 border-b-2 border-fuchsia-100 bg-yellow-900 md:w-1/2">
        <div className="relative z-10 flex flex-col items-center justify-center gap-4 rounded border-2 border-yellow-300 bg-yellow-600 p-8 md:p-16">
          <GridZigzag className="absolute -left-2 -top-2 stroke-yellow-900 stroke-2" />
          <h1 className="relative text-xl font-bold text-yellow-300 md:text-4xl">
            Login to{" "}
            <Mookbark className="-mr-1 inline stroke-2 hover:animate-ping hover:grayscale-0" />
            ookbark! 🗝️
            <HorizontalZigzag2 className="absolute top-0 w-16 stroke-yellow-900 stroke-2" />
          </h1>
          <p className="text-md md:text-lg">
            Manage and talk to your bookmarks.
          </p>
          <p className="mt-10 font-semibold">Login with</p>
          <div className="flex items-center justify-center gap-2">
            <Button
              onClick={() => {
                handleLogin("google");
              }}
            >
              Google
            </Button>
            <Button
              onClick={() => {
                handleLogin("discord");
              }}
            >
              Discord
            </Button>
          </div>
        </div>
      </div>
      {/* <div id="faq" className="flex"> */}
      {/*   <Faq /> */}
      {/* </div> */}
      {/* <div className="relative z-10 flex flex-col gap-10 border-t-2 border-fuchsia-100 bg-yellow-900 py-10 text-center"> */}
      {/*   <p className="text-4xl">Catch you later...😘</p> */}
      {/*   <p> */}
      {/*     © Copyright Furkan Kalaycioglu {new Date().getFullYear()}. All Rights */}
      {/*     Reserved. */}
      {/*   </p> */}
      {/* </div> */}
      {width >= twScreens.md && rightEdges}
    </div>
  );
};

export default Login;
