import React, { useState } from 'react';
import init, { convert } from "wasm-lib";
import './App.css';

function App() {
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [result, setResult] = useState<string | null>(null);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0] || null;
    setSelectedFile(file);
  };

  const handleSubmit = async () => {
    if (!selectedFile) {
      alert("Please select a file before submitting!");
      return;
    }

    try {
      const text = await selectedFile.text();

      await init();

      const converted = convert(text);

      setResult(converted);
    } catch (error) {
      console.error("Error processing file: ", error);
      alert("Failed to process file. See console for details.");
    }
  };

  return (
    <div>
      <h1>LibreSplit Converter</h1>
      <div>
        <input type="file" accept=".lss" onChange={handleFileChange} />
        <button onClick={handleSubmit} disabled={!selectedFile}>Convert</button>
      </div>
      <div>
        {result && <p>Result: {result}</p>}
      </div>
    </div>
  );
}

export default App;
