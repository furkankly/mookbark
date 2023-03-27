import {
  // Navigate,
  Outlet,
  // useLocation
} from "react-router-dom";

const ProtectedRoute = ({
  // redirectPath = "/login",
  children,
}: {
  isAllowed?: boolean;
  redirectPath: string;
  children: JSX.Element;
}) => {
  // Checks are handled by axios interceptors as Cookies fit well with that a little bit more
  return children ? children : <Outlet />;
  // return <Navigate to={redirectPath} replace />;
};

export default ProtectedRoute;
