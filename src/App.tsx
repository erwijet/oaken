import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { api } from "@/lib/rpc";

function App() {
  const [name, setName] = useState("");

  return (
    <div>
      <Input value={name} onChange={e => setName(e.target.value)} />
      <Button onClick={() => api.query(["me", name]).then(it => alert(it))}>Go!</Button>
    </div>
  );
}

export default App;
