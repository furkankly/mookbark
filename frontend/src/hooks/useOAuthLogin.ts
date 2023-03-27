import { useEffect } from "react";

import { useMutation } from "@tanstack/react-query";
import axios from "axios";
import { useNavigate, useSearchParams } from "react-router-dom";

const useOAuthLogin = ({
  tokenUri,
  redirectUri,
  onSuccess,
  onError,
  isStateGeneratedOnClient = false,
}: {
  tokenUri: string;
  redirectUri: string;
  onSuccess?: (data: unknown) => void;
  onError?: (error: unknown) => void;
  isStateGeneratedOnClient?: boolean;
}) => {
  const [searchParams, _setSearchParams] = useSearchParams();
  const code = searchParams.get("code");
  const state = searchParams.get("state");

  const token = useMutation(
    async ({
      tokenUri,
      redirectUri,
      code,
      state,
    }: {
      tokenUri: string;
      redirectUri: string;
      code: string;
      state: string;
    }) => {
      const response = await axios.post(
        `${tokenUri}?redirect_uri=${redirectUri}&code=${code}&state=${state}`,
        undefined,
        {
          headers: {
            "Content-Type": "application/x-www-form-urlencoded",
          },
        }
      );
      return response.data;
    },
    { onSuccess, onError }
  );

  const navigate = useNavigate();

  useEffect(() => {
    if (isStateGeneratedOnClient) {
      const getToken = async () => {
        await token.mutateAsync({
          tokenUri,
          redirectUri,
          code: code as string,
          state: state as string,
        });
        sessionStorage.removeItem("state");
        navigate("/", { replace: true });
      };
      const expectedState = sessionStorage.getItem("state");
      const isStatePassed = state === expectedState;
      if (code && isStatePassed) getToken();
    } else {
      const getToken = async () => {
        await token.mutateAsync({
          tokenUri,
          redirectUri,
          code: code as string,
          state: state as string,
        });
      };
      getToken();
    }
  }, [isStateGeneratedOnClient, tokenUri, redirectUri, code, state]);
};

export default useOAuthLogin;
