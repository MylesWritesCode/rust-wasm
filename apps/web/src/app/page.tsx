import { log } from "@repo/logger";
import { CounterButton, Link } from "@repo/ui";

export const metadata = {
  title: "Store | Kitchen Sink",
};

export default function Store(): JSX.Element {
  log("Hey! This is the Store page.");

  return (
    <div className="container">
      <h1 className="title">
        <span>rust-wasm</span>
      </h1>
    </div>
  );
}
