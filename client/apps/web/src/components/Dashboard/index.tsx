import { useSearchParams } from "react-router-dom";
import AnimatedMenu from "components/AnimatedMenu";
import Bookmarks from "../Bookmarks";
import Bookmark from "../Bookmark";

const Dashboard = () => {
  const [searchParams, _setSearchParams] = useSearchParams();
  const bookmarkUrl = searchParams.get("bookmark_url");

  return (
    <main className="relative flex h-full flex-col overflow-auto bg-yellow-900">
      {/* <Mookbark className="fixed left-1/2 top-1/2 h-full w-auto -translate-x-1/2 -translate-y-1/2 opacity-10" /> */}
      <div className="relative w-full shrink-0 basis-16 border-b-2 border-b-fuchsia-100 bg-yellow-300">
        <AnimatedMenu />
      </div>
      {bookmarkUrl ? <Bookmark /> : <Bookmarks />}
    </main>
  );
};

export default Dashboard;
