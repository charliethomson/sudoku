import { createRootRoute, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { Suspense } from "react";
import Spinner from "../components/Spinner";

export const Route = createRootRoute({
  component: () => (
    <main
      id="app-root"
      className="w-screen h-screen overflow-hidden flex justify-center items-center p-16 bg-slate-800 dark:bg-slate-900 text-slate-800 dark:text-slate-100"
    >
      <div
        id="app-container"
        className="w-full h-fit min-h-[300px] max-h-full p-16 overflow-auto bg-slate-100 dark:bg-slate-700 rounded-xl"
      >
        <Suspense
          fallback={
            <div>
              <Spinner />
            </div>
          }
        >
          <Outlet />
        </Suspense>
      </div>
      <TanStackRouterDevtools />
    </main>
  ),
});
