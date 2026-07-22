import { useState } from "react";
import Overlay from "./components/Overlay/Overlay";

function App() {
  const [isOpen, setIsOpen] = useState(true);

  return (
    <div className="min-h-screen flex items-center justify-center">
      <Overlay isOpen={isOpen} onClose={() => setIsOpen(false)} />
    </div>
  );
}

export default App;
