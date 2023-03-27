"use client";

import { useRouter } from "next/navigation";
import { Button } from "ui";
export default function LoginButton() {
  const router = useRouter();
  return (
    <Button
      className="mr-2 px-10 font-semibold"
      onClick={() => {
        router.push("/dashboard/login");
      }}
    >
      Login
    </Button>
  );
}
