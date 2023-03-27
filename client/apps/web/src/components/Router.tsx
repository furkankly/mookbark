import { createBrowserRouter } from "react-router-dom";
import Login from "components/Login";
import Dashboard from "components/Dashboard";
import NoMatch from "components/NoMatch";
import ProtectedRoute from "components/ProtectedRoute";
// import Faq from "components/Faq";

const routes = [
  {
    path: "/login",
    element: <Login />,
  },
  {
    path: "/",
    element: (
      <ProtectedRoute redirectPath="/login">
        <Dashboard />
      </ProtectedRoute>
    ),
  },
  {
    path: "*",
    element: <NoMatch />,
  },
];

const router = createBrowserRouter(routes, { basename: "/dashboard" });

export default router;
