import { createClient } from "@rspc/client";
import { TauriTransport } from "@rspc/tauri";

import type { Procedures } from "@/bindings";

export const api = createClient<Procedures>({
  transport: new TauriTransport(),
});
