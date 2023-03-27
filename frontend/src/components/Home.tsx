import { useQuery } from "@tanstack/react-query";
import axios from "axios";

import Watermark from "assets/Watermark.svg";

function Home() {
  const { data } = useQuery(["bookmarks"], async () => {
    let result = await axios.get("https://127.0.0.1:4444/bookmarks");
    return result;
  });

  return (
    <main>
      <div className="w-full p-4 bg-yellow-300">
        <img className="h-full" src={Watermark} />
      </div>
      <pre>{data ? JSON.stringify(data.data, null, 2) : null}</pre>
    </main>
  );
}

export default Home;
