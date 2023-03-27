import { useSyncExternalStore } from "react";

function useLocalStorage(key: string) {
  const subscribe = (listener: (this: Window, ev: StorageEvent) => any) => {
    window.addEventListener("storage", listener);
    return () => {
      window.removeEventListener("storage", listener);
    };
  };
  const getSnapShot = () => {
    return localStorage.getItem(key);
  };
  const value = useSyncExternalStore(subscribe, getSnapShot);
  return value ? JSON.parse(value) : null;
}

export default useLocalStorage;
