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
    <div className="min-h-screen bg-gray-100 flex items-center justify-center p-6">
      <div className="max-w-lg w-full bg-white rounded-lg shadow-lg p-6 space-y-6">
        <h1 className="text-2xl font-bold text-center text-gray-800">
          LibreSplit Converter
        </h1>
        <div className="space-y-4">
          <input
            type="file"
            accept=".lss"
            onChange={handleFileChange}
            className="block w-full px-3 py-2 border border-gray-300 rounded-md text-gray-600 focus:outline-none focus:ring focus:ring-indigo-500"
          />
          <button
            onClick={handleSubmit}
            disabled={!selectedFile}
            className={
              'w-full px-4 py-2 text-black font-semibold rounded-md ${selectedFile ? "bg-indigo-600 hover:bg-indigo-700" : "bg-gray-400 cursor-not-allowed"}'
            }
          >
            Convert
          </button>
        </div>
        {result && (
          <div className="space-y-4">
            <p className="text-green-600 font-medium text-center">
              Conversion successful! Click the button below to download your
              LibreSplit file.
            </p>
            <button
              onClick={handleDownload}
              className="w-full px-4 py-2 bg-green-600 text-white font-semibold rounded-md hover:bg-green-700"
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
