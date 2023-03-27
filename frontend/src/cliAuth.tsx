import React, { useState } from "react";
import ReactDOM from "react-dom/client";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import "./index.css";
import useOAuthLogin from "hooks/useOAuthLogin";

const Auth = () => {
  const [isAuthSuccessful, setIsAuthSuccessful] = useState<boolean | null>(
    null
  );
  useOAuthLogin({
    tokenUri: "https://localhost:5172/token",
    redirectUri: "https://localhost:5172/login",
    onSuccess: () => {
      setIsAuthSuccessful(true);
    },
    onError: () => {
      setIsAuthSuccessful(false);
    },
    isStateGeneratedOnClient: false,
  });
  return (
    <div>
      {isAuthSuccessful === null
        ? "Authenticating..."
        : isAuthSuccessful
        ? "Authenticated! You can close this browser tab."
        : "Authentication failed!"}
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
      <div className="h-screen flex justify-center items-center text-4xl text-yellow-900">
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
