import init, { Demo } from "./pkg/glemu_wasm_pack_demo.js";

const canvas = document.getElementById("canvas");

function resizeCanvas() {
  const dpr = window.devicePixelRatio || 1;
  const width = Math.max(1, Math.floor(canvas.clientWidth * dpr));
  const height = Math.max(1, Math.floor(canvas.clientHeight * dpr));

  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width;
    canvas.height = height;
  }
}

async function main() {
  await init();
  resizeCanvas();
  const gl = canvas.getContext("webgl2");

  if (!gl) {
    throw new Error("WebGL2 is not available in this browser");
  }

  const demo = new Demo(gl);

  function frame(timeMs) {
    resizeCanvas();
    demo.render_frame(timeMs);
    window.requestAnimationFrame(frame);
  }

  window.requestAnimationFrame(frame);
  window.addEventListener("resize", resizeCanvas);
}

main().catch((error) => {
  console.error(error);
});
