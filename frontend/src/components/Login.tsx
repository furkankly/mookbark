import useOAuthLogin from "hooks/useOAuthLogin";
import useWindowDimensions from "hooks/useWindowDimensions";
import Zigzag from "./Zigzag";
import ButtonZigzag from "./ButtonZigzag";
import HeaderZigzag from "./HeaderZigzag";
import GridZigzag from "./GridZigzag";
import Mookbark from "./Mookbark";

const Login = () => {
  const state = self.crypto.randomUUID();
  if (!sessionStorage.getItem("state")) sessionStorage.setItem("state", state);

  useOAuthLogin({
    tokenUri: "https://localhost:4444/token",
    redirectUri: "http://localhost:5173/login",
    onSuccess: (data) => {
      localStorage.setItem("mookbark.auth", JSON.stringify(data));
    },
    isStateGeneratedOnClient: true,
  });

  const authUrl = `https://discord.com/api/oauth2/authorize?client_id=1062682150912151602&redirect_uri=http%3A%2F%2Flocalhost%3A5173%2Flogin&response_type=code&scope=identify&state=${sessionStorage.getItem(
    "state"
  )}`;

  const handleDiscordLogin = () => {
    window.open(authUrl, "_self");
  };

  const [_width, height] = useWindowDimensions();
  const amount = Math.floor(height / 192);

  const leftEdges = (
    <div className="absolute top-0 left-60 h-full flex flex-col items-center justify-center">
      {Array.from(Array(amount)).map((_el, index) => (
        <Zigzag className="stroke-gray-800 stroke-2" key={index} />
      ))}
    </div>
  );
  const rightEdges = (
    <div className="absolute top-0 right-60 h-full flex flex-col items-center justify-center">
      {Array.from(Array(amount)).map((_el, index) => (
        <Zigzag className="stroke-gray-800 stroke-2" key={index} />
      ))}
    </div>
  );

  return (
    <div className="relative bg-yellow-600">
      <Mookbark className="absolute left-96 top-32 w-8 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      <Mookbark className="absolute left-2/4 top-10 w-5 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      <Mookbark className="absolute left-80 top-48 w-7 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      <Mookbark className="absolute left-1/3 top-64 w-3 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      <Mookbark className="absolute left-2/3 top-80 w-8 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      <Mookbark className="absolute left-72 bottom-64 w-8 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      <Mookbark className="absolute right-1/4 bottom-10 w-10 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      <Mookbark className="absolute right-2/4 bottom-24 w-6 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      <Mookbark className="absolute right-3/4 bottom-40 w-6 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      <Mookbark className="absolute right-1/3 bottom-56 w-2 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      <Mookbark className="absolute right-72 bottom-72 w-8 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      <Mookbark className="absolute right-96 bottom-48 w-8 rotate-12 grayscale hover:grayscale-0 hover:animate-ping" />
      {leftEdges}
      <div className="bg-gray-400 mx-64 flex flex-col justify-center items-center h-screen">
        <div className="relative bg-yellow-600 p-16 flex flex-col justify-center items-center gap-4 border-2 border-yellow-300 rounded">
          <GridZigzag className="absolute -top-2 -left-2 stroke-yellow-900 stroke-2" />
          <h1 className="relative text-yellow-300 text-4xl font-bold">
            Login to{" "}
            <Mookbark className="-mr-1 inline hover:grayscale-0 hover:animate-ping stroke-2" />
            ookbark! 🗝️
            <HeaderZigzag className="absolute top-0 stroke-yellow-900 w-16 stroke-2" />
          </h1>
          <p className="text-white text-lg">
            Start managing your bookmarks with ease.
          </p>
          <button
            className="relative mt-10 p-2 bg-yellow-900 hover:bg-yellow-300 group text-yellow-300 hover:text-yellow-900 focus:ring-1 ring-inset ring-yellow-300 rounded"
            onClick={handleDiscordLogin}
          >
            <ButtonZigzag className="absolute -bottom-1 left-0 stroke-yellow-300 group-hover:stroke-yellow-900 stroke-2 w-16" />
            Login with Discord
          </button>
        </div>
      </div>
      {rightEdges}
    </div>
  );
};

export default Login;
