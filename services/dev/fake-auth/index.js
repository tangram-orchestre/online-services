import http from "http";
import fs from "fs";
import path from "path";

const PORT = 3000;
const __dirname = path.dirname(new URL(import.meta.url).pathname);
const CURRENT_USER_FILE = path.join(__dirname, "current-user.json");
const USERS_FILE = path.join(__dirname, "users.json");

function getSampleFileName(filename) {
  const ext = path.extname(filename);
  const baseName = path.basename(filename, ext);
  const dirName = path.dirname(filename);
  return path.join(dirName, `${baseName}.sample${ext}`);
}

// Function to read key-value pairs from a file
function readFile(filename) {
  if (!fs.existsSync(filename)) {
    try {
      fs.copyFileSync(getSampleFileName(filename), filename);
      console.log(`${filename} missing. Created from sample file.`);
    } catch (error) {
      console.error("Error creating file from sample:", error);
      return {};
    }
  }

  try {
    const data = fs.readFileSync(filename, "utf-8");
    return JSON.parse(data);
  } catch (error) {
    console.error("Error reading headers file:", error);
    return {};
  }
}

function getUser(id) {
  const users = readFile(USERS_FILE);
  return users.find((user) => user.pk === id) || null;
}

// Create an HTTP server
const server = http.createServer((req, res) => {
  try {
    if (req.url === "/auth" && req.method === "GET") {
      const userId = readFile(CURRENT_USER_FILE);
      if (userId === null) {
        res.writeHead(403);
        res.end();
        return;
      }

      const user = getUser(userId);
      if (user === null) {
        res.writeHead(403);
        res.end();
        return;
      }

      res.setHeader("X-App-UserId", userId);
      res.setHeader("X-App-User", user.username);
      res.setHeader("X-App-Email", user.email);
      res.setHeader(
        "X-App-Groups",
        user.groups_obj.map((g) => g.name).join(",")
      );
      res.setHeader("X-App-LastName", user.attributes.last_name);
      res.setHeader("X-App-FirstName", user.attributes.first_name);
      res.setHeader("X-App-PhoneNumber", user.attributes.phone_number);
      res.writeHead(200, { "Content-Type": "application/json" });
      res.end();
      return;
    }

    res.writeHead(404, { "Content-Type": "text/plain" });
    res.end("Not Found");
  } catch (error) {
    console.error("Error processing request");
    res.writeHead(500, { "Content-Type": "text/plain" });
    res.end("Internal Server Error");
  }
});

// Start the server
server.listen(PORT, () => {
  console.log(`Server is running on http://localhost:${PORT}`);
});
