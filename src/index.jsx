/* @refresh reload */
import { render } from "solid-js/web";
import "./index.css";
import { commands } from "./bindings";
import { AppRouter } from "./router";

render(() => {
  setSystemColor();
  return <AppRouter />;
}, document.getElementById("root"));

function arrayToRGBA(arr) {
  if (arr.length !== 4) throw new Error("Invalid array length");
  return `rgba(${arr[0]}, ${arr[1]}, ${arr[2]}, ${arr[3]})`;
}
async function setSystemColor() {
  const color = await commands.getAccentColor();
  if (color.data) {
    document.documentElement.style.cssText = `--input-accent-color: ${arrayToRGBA(color.data)}`;
  }
}
