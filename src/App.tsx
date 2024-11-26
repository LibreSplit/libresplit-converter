import React, { useEffect, useState } from 'react';
import init, { add } from "wasm-lib";
import './App.css';

function App() {
  const [ans, setAns] = useState(0);
  useEffect(() => {
    init().then(() => {
      setAns(add(1, 1));
    })
  }, [])

  return (
    <p>1 + 1 = {ans}</p>
  );
}

export default App;
