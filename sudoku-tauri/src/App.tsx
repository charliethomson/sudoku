import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { RouterProvider, createRouter } from "@tanstack/react-router";
const queryClient = new QueryClient();
import { routeTree } from "./routeTree.gen";
import ThemeManager from "./components/ThemeManager";

const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
      <ThemeManager />
    </QueryClientProvider>
  );
}

export default App;
