import { useMutation, useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { useLocation, useNavigate, useSearchParams } from "react-router-dom";
import axios from "utils/axios";

// isCli=true indicates that its the CLI OAuth flow, and the state is not generated on the browser
const useOAuthLogin = ({
  onSuccess,
  isCli = false,
}: {
  onSuccess?: (data: unknown) => void;
  isCli?: boolean;
}) => {
  const [searchParams, _setSearchParams] = useSearchParams();
  const code = searchParams.get("code");
  const state = searchParams.get("state");
  const oauth_provider = searchParams.get("oauth_provider");

  const { mutateAsync, status: statusToken } = useMutation({
    mutationFn: async ({
      url,
      redirectUrl,
      code,
      state,
    }: {
      url: string;
      redirectUrl: string;
      code: string;
      state: string;
    }) => {
      // Even though we are ahead of the first step (code is received using auth url which includes the redirect_uri itself, oauth providers still need this for the token exchange for some reason)
      const response = await axios.post(
        `${url}?redirect_uri=${redirectUrl}&code=${code}&state=${state}`
      );
      return response.data;
    },
    onSuccess,
  });

  const location = useLocation();
  const navigate = useNavigate();
  const { status: statusCheckAuth } = useQuery({
    queryKey: ["checkAuth"],
    queryFn: () => {
      const response = axios.get<unknown>("/check-auth");
      return response;
    },
    enabled: !isCli && searchParams.size === 0 && location.hash === "",
    onSuccess: () => {
      navigate("/");
    },
    retry: false,
  });

  useEffect(() => {
    const getToken = async () => {
      const baseUrl = new URL(window.location.href).origin;
      const url = isCli ? baseUrl + "/api/token" : "/token";
      const redirectUrl = `${baseUrl}${
        isCli ? "" : "/dashboard"
      }/login?oauth_provider=${oauth_provider}`;

      await mutateAsync({
        url,
        redirectUrl,
        code: code as string,
        state: state as string,
      });
    };

    if (isCli) {
      getToken();
    } else {
      const expectedState = sessionStorage.getItem("state");
      const isStatePassed = state === expectedState;
      if (code && isStatePassed) getToken();
    }
  }, [statusCheckAuth, isCli, oauth_provider, code, state]);

  return { statusToken };
};

export default useOAuthLogin;
