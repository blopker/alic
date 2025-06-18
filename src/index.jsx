/* @refresh reload */
import { render } from "solid-js/web";
import "./index.css";
import { AppRouter } from "./router";

render(() => <AppRouter />, document.getElementById("root"));
