import http from "http";
import fs from "fs";
import path from "path";

const PORT = 3000;
const __dirname = path.dirname(new URL(import.meta.url).pathname);
const HEADERS_FILE = path.join(__dirname, "headers.json");
const SAMPLE_HEADERS_FILE = path.join(__dirname, "headers.sample.json");

// Function to read key-value pairs from a file
function readHeadersFromFile() {
  if (!fs.existsSync(HEADERS_FILE)) {
    try {
      fs.copyFileSync(SAMPLE_HEADERS_FILE, HEADERS_FILE);
      console.log("Headers file missing. Created from sample file.");
    } catch (error) {
      console.error("Error creating headers file from sample:", error);
      return {};
    }
  }

  try {
    const data = fs.readFileSync(HEADERS_FILE, "utf-8");
    return JSON.parse(data);
  } catch (error) {
    console.error("Error reading headers file:", error);
    return {};
  }
}

// Create an HTTP server
const server = http.createServer((req, res) => {
  if (req.method === "GET") {
    const headers = readHeadersFromFile();

    // Set headers from the file
    for (const [key, value] of Object.entries(headers)) {
      res.setHeader(key, value);
    }

    res.writeHead(200, { "Content-Type": "text/plain" });
    res.end("Headers set from file.\n");
  } else {
    res.writeHead(405, { "Content-Type": "text/plain" });
    res.end("Method not allowed.\n");
  }
});

// Start the server
server.listen(PORT, () => {
  console.log(`Server is running on http://localhost:${PORT}`);
});
