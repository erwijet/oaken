import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router-dom";

import "./index.css";
import { router } from "./router";
import { Invalidator } from "./Invalidator";
import { PageLoader } from "@/PageLoader";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={new QueryClient()}>
      <Invalidator>
        <PageLoader>
          <RouterProvider router={router} />
        </PageLoader>
      </Invalidator>
    </QueryClientProvider>
  </React.StrictMode>,
);
