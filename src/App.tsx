import React, { useState } from "react";

import init, { convert } from "wasm-lib";

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

  const handleDownload = () => {
    if (!result || !selectedFile) return;

    const fileName = selectedFile.name.replace(/\.[^/.]+$/, ".json");

    const blob = new Blob([result], { type: "application/json" });
    const url = URL.createObjectURL(blob);

    const link = document.createElement("a");
    link.href = url;
    link.download = fileName;
    link.click();

    URL.revokeObjectURL(url);
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-linear-to-tr from-gray-700 to-sky-900 p-6">
      <div className="w-full max-w-lg space-y-6 rounded-lg bg-gray-800 p-6 shadow-lg">
        <h1 className="text-center text-2xl font-bold text-white">
          LibreSplit Converter
        </h1>
        <div className="space-y-4">
          <input
            type="file"
            accept=".lss"
            onChange={handleFileChange}
            className="block w-full rounded-md border border-white px-3 py-2 text-white focus:ring focus:ring-indigo-500 focus:outline-none"
          />
          <button
            onClick={handleSubmit}
            disabled={!selectedFile}
            className={
              '${selectedFile ? "bg-indigo-600 hover:bg-indigo-700" : "bg-gray-400 cursor-not-allowed"} w-full rounded-md px-4 py-2 font-semibold text-white'
            }
          >
            Convert
          </button>
        </div>
        {result && (
          <div className="space-y-4">
            <p className="text-center font-medium text-green-600">
              Conversion successful! Click the button below to download your
              LibreSplit file.
            </p>
            <button
              onClick={handleDownload}
              className="w-full rounded-md bg-green-600 px-4 py-2 font-semibold text-white hover:bg-green-700"
            >
              Download
            </button>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
