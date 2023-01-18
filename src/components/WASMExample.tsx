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
      <button
        onClick={() => {
          let output = ctx.wasm.runCairoProgram(25, 60, 40);
          let x_position = output.position[0].map((big: [number, number[]]) => {
            let sum = BigInt(0);
            big[1].forEach((x: number, i) => {
              sum += BigInt(x) * BigInt(2 ** (32 * i));
            });
            return sum;
          });
          console.log(x_position);
          let y_position = output.position[1].map((big: [number, number[]]) => {
            let sum = BigInt(0);
            big[1].forEach((x: number, i) => {
              sum += BigInt(x) * BigInt(2 ** (32 * i));
            });
            return sum;
          });
          console.log(y_position);
        }}
      >
        Click me
      </button>
    </div>
  );
};
