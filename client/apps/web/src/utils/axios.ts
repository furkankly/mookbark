import axios, { isAxiosError } from "axios";
import router from "components/Router";

const instance = axios.create({
  baseURL: import.meta.env.VITE_SERVER_URL,
  withCredentials: import.meta.env.DEV,
});

instance.interceptors.response.use(undefined, async (error: unknown) => {
  if (isAxiosError(error)) {
    if (error.response) {
      if (error.response.status === 401) {
        // for cliAuth router module is included and navigation happens but it doesnt matter
        router.navigate("/login", { replace: true });
      }
    }
    // React query needs thrown errors
    return Promise.reject(error);
  }
});
export default instance;
