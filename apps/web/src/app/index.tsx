import React from "react";
import "./styles.css";
import { CounterButton, Link } from "@repo/ui";
import { greet } from "metamorph";

function App(): JSX.Element {
  const fetchData = async () => {
    const input = { vertices: 20, edges: 20 };
    const data = await fetch("http://localhost:5001/generate-graph", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(input),
    });

    const json = await data.json();
    console.log(json);
  };

  return (
    <div className="container">
      <h1 className="title">
        Admin <br />
        <span>Kitchen Sink</span>
      </h1>
      <CounterButton />
      <button type="button" onClick={fetchData}>
        fetch graph
      </button>
      <button type="button" onClick={() => greet("hello there general kenobi")}>
        some button
      </button>
      <p className="description">
        Built With{" "}
        <Link href="https://turbo.build/repo" newTab>
          Turborepo
        </Link>
        {" & "}
        <Link href="https://vitejs.dev/" newTab>
          Vite
        </Link>
      </p>
    </div>
  );
}

export default App;
