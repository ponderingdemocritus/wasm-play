import { useContext, useEffect } from "react";
import { WASMContext } from "../context/WASM";

export const WASMExample = () => {
  const ctx = useContext(WASMContext);

  useEffect(() => {
    // Initialize wasm env.
    if (ctx.wasm) {
    }
    // ctx.wasm.start();
  }, [ctx]);

  if (!ctx.wasm) {
    return <>...</>;
  }

  return (
    <div>
      Computed from WASM: 4+3=
      <button onClick={() => ctx.wasm.runCairoProgram(2, 2, 2)}>
        Click me
      </button>
    </div>
  );
};
