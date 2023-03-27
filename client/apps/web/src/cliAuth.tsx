import React from "react";
import ReactDOM from "react-dom/client";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { Mookbark } from "ui";
import useOAuthLogin from "hooks/useOAuthLogin";
import "./index.css";

const Auth = () => {
  const status = useOAuthLogin({
    isCli: true,
  });

  return (
    <div className="flex h-full flex-col items-center justify-center bg-gray-400 font-bold text-yellow-900">
      <Mookbark className="-mr-1 inline stroke-2 hover:animate-ping hover:grayscale-0" />
      {status.statusToken === "loading"
        ? "Authenticating..."
        : status.statusToken === "error"
          ? "Authentication failed!"
          : "Authenticated! You can close this browser tab."}
    </div>
  );
};

const routes = [
  {
    path: "/login",
    element: <Auth />,
  },
  {
    path: "*",
    element: (
      <div className="flex h-screen items-center justify-center text-4xl text-yellow-900">
        404 Page Not Found
      </div>
    ),
  },
];

const router = createBrowserRouter(routes);
const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  </React.StrictMode>
);
