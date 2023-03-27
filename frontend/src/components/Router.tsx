import { createBrowserRouter } from "react-router-dom";

import Login from "components/Login";
import Home from "components/Home";
import ProtectedRoute from "components/ProtectedRoute";

const routes = [
  {
    path: "/login",
    element: <Login />,
  },
  {
    path: "/",
    // TODO: Improve isAllowed
    element: (
      <ProtectedRoute
        isAllowed={!!localStorage.getItem("mookbark.auth")}
        redirectPath="/login"
      >
        <Home />
      </ProtectedRoute>
    ),
  },
];

const router = createBrowserRouter(routes);

export default router;
